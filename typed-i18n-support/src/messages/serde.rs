use crate::diagnostic::Diagnostic;
use crate::messages::messages::Messages;
use crate::messages::raw::RawMessages;
use crate::messages::MessagesAsTree;
use indexmap::IndexMap;
use proc_macro2::Span;
use std::borrow::Cow;

#[derive(serde::Deserialize)]
pub(crate) struct SerdeInput<'a> {
    #[serde(rename = "_version", default = "default_version")]
    version: usize,
    #[serde(borrow, flatten)]
    inner: IndexMap<Cow<'a, str>, ObjectOrString<'a>>,
}

fn default_version() -> usize {
    2
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ObjectOrString<'a> {
    LangMap(IndexMap<Cow<'a, str>, Cow<'a, str>>),
    Depth(IndexMap<Cow<'a, str>, Box<ObjectOrString<'a>>>),
}

impl<'a> ObjectOrString<'a> {
    fn into_outer<D: Diagnostic>(
        self,
        diagnostic: &mut D,
        span: Span,
        separator: &str,
        result: &mut RawMessages<'a>,
        keys: &mut Vec<Cow<'a, str>>,
    ) {
        match self {
            ObjectOrString::LangMap(l) => {
                let key = if keys.len() == 1 {
                    keys[0].clone()
                } else {
                    Cow::Owned(keys.join(separator))
                };
                if result.0.insert(key.clone(), (keys.clone(), l)).is_some() {
                    diagnostic.emit_error(
                        span,
                        format!(r#"duplicate key "{key}", created from {keys:?}"#),
                    );
                }
            }
            ObjectOrString::Depth(d) => {
                for (k, v) in d {
                    keys.push(k);
                    v.into_outer(diagnostic, span, separator, result, keys);
                    keys.pop();
                }
            }
        }
    }
}

impl<'a> RawMessages<'a> {
    pub(crate) fn parse_serde<D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        separator: &str,
        key_map: SerdeInput<'a>,
    ) -> RawMessages<'a> {
        if key_map.version != 2 {
            diagnostic.emit_error(span, "_version is not 2");
        }
        let mut result = RawMessages(IndexMap::new());
        key_map.inner.into_iter().for_each(|(k, v)| {
            v.into_outer(diagnostic, span, separator, &mut result, &mut vec![k]);
        });

        result
    }
}

#[derive(::serde::Serialize)]
#[serde(untagged)]
pub(super) enum MessagesTreeInner<'a> {
    Object(IndexMap<&'a str, Box<MessagesTreeInner<'a>>>),
    String(&'a str),
}

impl<'a> MessagesTreeInner<'a> {
    fn object(&mut self) -> &mut IndexMap<&'a str, Box<MessagesTreeInner<'a>>> {
        match self {
            MessagesTreeInner::Object(o) => o,
            MessagesTreeInner::String(_) => panic!("called object() on a String"),
        }
    }
}

impl<'a> Messages<'a> {
    #[must_use]
    pub fn as_tree(&'a self) -> MessagesAsTree<'a> {
        let mut inner: IndexMap<&str, Box<MessagesTreeInner>> = IndexMap::new();
        for (_, m) in self {
            let mut ptr = &mut inner;
            for p in m.path() {
                ptr = ptr
                    .entry(p.as_ref())
                    .or_insert_with(|| Box::new(MessagesTreeInner::Object(IndexMap::new())))
                    .object();
            }
            for (l, mkl) in &m.message_lines {
                ptr.insert(l, Box::new(MessagesTreeInner::String(mkl.borrow_line())));
            }
        }

        MessagesAsTree {
            _version: 2,
            inner: MessagesTreeInner::Object(inner),
        }
    }
}
