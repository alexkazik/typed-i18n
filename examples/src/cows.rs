extern crate alloc;

use alloc::borrow::Cow;
use typed_i18n::{Builder, BuilderFromRef, BuilderFromValue};

pub struct Cows(Vec<Cow<'static, str>>);

impl Builder for Cows {
    type Output = Vec<Cow<'static, str>>;

    #[inline]
    fn empty() -> Self::Output {
        Vec::new()
    }

    #[inline]
    fn const_str(i: &'static str) -> Self::Output {
        vec![Cow::Borrowed(i)]
    }

    #[inline]
    fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn push_const_str(mut self, i: &'static str) -> Self {
        self.0.push(Cow::Borrowed(i));
        self
    }

    #[inline]
    fn push_str(mut self, i: &str) -> Self {
        self.0.push(Cow::Owned(i.to_string()));
        self
    }

    #[inline]
    fn finish(self) -> Self::Output {
        self.0
    }
}

// This probably should not be used as it's better to use clone directly
// to make clear that it's used. It's here for demonstration purposes.
impl BuilderFromRef<Cow<'static, str>> for Cows {
    #[inline]
    fn push(mut self, i: &Cow<'static, str>) -> Self {
        self.0.push(i.clone());
        self
    }
}

impl BuilderFromValue<Cow<'static, str>> for Cows {
    #[inline]
    fn push(mut self, i: Cow<'static, str>) -> Self {
        self.0.push(i);
        self
    }
}
