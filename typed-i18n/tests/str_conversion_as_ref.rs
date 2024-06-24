#![cfg(feature = "alloc")]
use crate::derive::Language;
use std::borrow::Cow;

mod derive {
    #![no_implicit_prelude]

    use ::std::string::String;
    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "mixed_str", str_conversion = "as_ref")]
    pub enum Language {
        En,
        #[allow(dead_code)]
        De,
    }
}

#[test]
fn input_conversion_as_ref() {
    assert_eq!(
        Language::En.hello_you(Cow::Owned("you".to_string())),
        "Hello you".to_string()
    );
}
