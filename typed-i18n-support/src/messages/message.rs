use crate::messages::message_line::MessageLine;
use crate::messages::param_type::ParamType;
use indexmap::IndexMap;
use std::borrow::Cow;

#[derive(::serde::Serialize)]
pub struct Message<'a> {
    #[serde(skip)]
    pub(crate) path: Vec<Cow<'a, str>>,
    #[serde(skip)]
    pub(crate) params: Vec<(String, ParamType)>,
    #[serde(flatten)]
    #[allow(clippy::struct_field_names)]
    pub(crate) message_lines: IndexMap<Cow<'a, str>, MessageLine<'a>>,
}

impl<'a, 'b: 'a> IntoIterator for &'b Message<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = MessageIter<'a, 'b>;
    fn into_iter(self) -> Self::IntoIter {
        MessageIter {
            inner: self.message_lines.iter(),
        }
    }
}

impl<'a> Message<'a> {
    #[must_use]
    pub fn iter<'b: 'a>(&'b self) -> MessageIter<'a, 'b> {
        self.into_iter()
    }

    #[must_use]
    pub fn path(&self) -> &[impl AsRef<str> + 'a] {
        &self.path
    }
}

pub struct MessageIter<'a, 'b> {
    inner: indexmap::map::Iter<'b, Cow<'a, str>, MessageLine<'a>>,
}

impl<'a, 'b: 'a> Iterator for MessageIter<'a, 'b> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(k, v)| (k.as_ref(), v.borrow_line().as_ref()))
    }
}
