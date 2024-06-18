use crate::messages::piece::Piece;
use ouroboros::self_referencing;
use serde::Serializer;
use std::borrow::Cow;

#[self_referencing]
pub(crate) struct MessageLine<'a> {
    pub(crate) line: Cow<'a, str>,
    #[borrows(line)]
    #[covariant]
    pub(crate) pieces: Vec<Piece<'this>>,
}

impl<'a> ::serde::Serialize for MessageLine<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.borrow_line())
    }
}

impl<'a> MessageLine<'a> {
    pub(crate) fn build<F>(line: Cow<'a, str>, pieces_builder: F) -> Self
    where
        F: for<'b> FnOnce(&'b str) -> Vec<Piece<'b>>,
    {
        MessageLineBuilder {
            line,
            pieces_builder: |l| pieces_builder(l),
        }
        .build()
    }
}
