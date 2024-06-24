#![cfg(feature = "alloc")]
use crate::common::{Element, Output, Tester};
use crate::derive::Language;

mod common;

mod derive {
    #![no_implicit_prelude]

    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "_")]
    pub enum Language {
        En,
        De,
    }
}

#[test]
fn generic_builder() {
    assert_eq!(
        Language::En.hello_you::<Tester<()>>("you"),
        Output::Built(vec![
            Element::Const("Hello "),
            Element::String("you".to_string()),
        ])
    );

    assert_eq!(
        Language::De.hello_you::<String>("du"),
        "Hallo du".to_string()
    );
}
