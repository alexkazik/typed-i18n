extern crate alloc;

use crate::Builder;
use alloc::borrow::Cow;
use alloc::string::{String, ToString};

impl Builder for String {
    type Output = String;

    #[inline]
    fn empty() -> Self::Output {
        String::new()
    }

    #[inline]
    fn const_str(i: &'static str) -> Self::Output {
        i.to_string()
    }

    #[inline]
    fn new() -> Self {
        String::new()
    }

    #[inline]
    fn push_str(mut self, i: &str) -> Self {
        String::push_str(&mut self, i);
        self
    }

    #[inline]
    fn finish(self) -> String {
        self
    }
}

impl Builder for Cow<'static, str> {
    type Output = Cow<'static, str>;

    #[inline]
    fn const_str(i: &'static str) -> Self::Output {
        Cow::Borrowed(i)
    }

    #[inline]
    fn new() -> Self {
        Cow::Owned(String::new())
    }

    #[inline]
    fn push_str(self, i: &str) -> Self {
        match self {
            Cow::Borrowed(b) => {
                // this never happens due to `typed-i18n` calls, but could be otherwise
                let mut o = b.to_string();
                String::push_str(&mut o, i);
                Cow::Owned(o)
            }
            Cow::Owned(mut o) => {
                String::push_str(&mut o, i);
                Cow::Owned(o)
            }
        }
    }

    #[inline]
    fn finish(self) -> Self::Output {
        self
    }
}
