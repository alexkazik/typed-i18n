use crate::attribute::Parameters;
use crate::diagnostic::{Diagnostic, Simulated};
use crate::languages::Languages;
use crate::messages::message::Message;
use crate::messages::raw::RawMessages;
use crate::messages::serde::{MessagesTreeInner, SerdeInput};
use indexmap::IndexMap;
use proc_macro2::Span;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;

#[derive(::serde::Serialize)]
pub struct Messages<'a> {
    _version: usize,
    #[serde(flatten)]
    inner: IndexMap<Cow<'a, str>, Message<'a>>,
}

#[allow(clippy::into_iter_without_iter)] // because of a false positive
impl<'a, 'b: 'a> IntoIterator for &'b Messages<'a> {
    type Item = (&'a str, &'b Message<'a>);
    type IntoIter = MessagesIter<'a, 'b>;
    fn into_iter(self) -> Self::IntoIter {
        MessagesIter {
            inner: self.inner.iter(),
        }
    }
}

impl<'a> Messages<'a> {
    #[must_use]
    pub(crate) fn new(inner: IndexMap<Cow<'a, str>, Message<'a>>) -> Self {
        Self { _version: 2, inner }
    }

    /// Simplified function to run the parser, will result in an error if any diagnostics are emitted.
    pub fn run_parse(
        parameters: &Parameters,
        languages: &Languages,
        content: &'a str,
    ) -> Result<Self, Simulated> {
        Simulated::run(|diagnostic| {
            Self::parse(
                diagnostic,
                Span::call_site(),
                parameters,
                languages,
                content,
            )
        })
    }

    #[must_use]
    pub fn parse<D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        parameters: &Parameters,
        languages: &Languages,
        content: &'a str,
    ) -> Self {
        let raw = match Path::new(&parameters.filename)
            .extension()
            .and_then(OsStr::to_str)
        {
            Some("lrc") => RawMessages::parse_lrc(diagnostic, span, content, languages),
            Some("yaml") => match serde_yaml::from_str::<SerdeInput>(content) {
                Ok(key_map) => {
                    RawMessages::parse_serde(diagnostic, span, &parameters.separator, key_map)
                }
                Err(err) => {
                    diagnostic.emit_error(span, format!("Invalid YAML format, {err}"));
                    RawMessages(IndexMap::default())
                }
            },
            Some("json") => match serde_json::from_str::<SerdeInput>(content) {
                Ok(key_map) => {
                    RawMessages::parse_serde(diagnostic, span, &parameters.separator, key_map)
                }
                Err(err) => {
                    diagnostic.emit_error(span, format!("Invalid JSON format, {err}"));
                    RawMessages(IndexMap::default())
                }
            },
            Some(ext) => {
                diagnostic.emit_error(span, format!("Unsupported file extension {ext:?}"));
                RawMessages(IndexMap::default())
            }
            None => {
                diagnostic.emit_error(
                    span,
                    format!("No file extension on {:?}", &parameters.filename),
                );
                RawMessages(IndexMap::default())
            }
        };
        diagnostic.should_abort_if_dirty();
        RawMessages::parse_values(diagnostic, span, raw, languages)
    }

    #[must_use]
    pub fn iter<'b: 'a>(&'b self) -> MessagesIter<'a, 'b> {
        self.into_iter()
    }
}

pub struct MessagesIter<'a, 'b> {
    inner: indexmap::map::Iter<'b, Cow<'a, str>, Message<'a>>,
}

impl<'a, 'b: 'a> Iterator for MessagesIter<'a, 'b> {
    type Item = (&'a str, &'b Message<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.as_ref(), v))
    }
}

/// Helper to save the messages in tree form with serde.
#[derive(::serde::Serialize)]
pub struct MessagesAsTree<'a> {
    pub(super) _version: usize,
    #[serde(flatten)]
    pub(super) inner: MessagesTreeInner<'a>,
}
