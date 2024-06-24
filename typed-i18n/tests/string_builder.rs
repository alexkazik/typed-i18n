#![cfg(feature = "alloc")]

use crate::derive::Language;
use typed_i18n::Builder;

mod derive {
    #![no_implicit_prelude]

    use crate::StringBuilder;
    use ::std::string::String;
    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "String", prefix = "s_")]
    #[typed_i18n(builder = "StringBuilder", prefix = "sb_")]
    pub enum Language {
        En,
        De,
    }
}

pub struct StringBuilder(String);

impl Builder for StringBuilder {
    type Output = String;

    fn new() -> Self {
        Self(String::new())
    }

    fn push_str(mut self, i: &str) -> Self {
        String::push_str(&mut self.0, i);
        self
    }

    fn finish(self) -> Self::Output {
        self.0
    }
}

#[test]
fn string_builder() {
    assert_eq!(Language::En.s_hello_world(), Language::En.sb_hello_world());
    assert_eq!(Language::De.s_hello_world(), Language::De.sb_hello_world(),);

    assert_eq!(
        Language::En.s_hello_you("you"),
        Language::En.sb_hello_you("you")
    );

    assert_eq!(Language::En.s_maybe_note(), Language::En.sb_maybe_note());
}
