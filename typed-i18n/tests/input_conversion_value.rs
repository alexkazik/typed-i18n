use crate::common::{Element, Output};
use crate::derive::Language;

mod common;

mod derive {
    #![no_implicit_prelude]

    use crate::common::Tester;
    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "Tester<u16>", input = "u16", input_conversion = "value")]
    pub enum Language {
        En,
        #[allow(dead_code)]
        De,
    }
}

#[test]
fn input_conversion_value() {
    assert_eq!(
        Language::En.hello_you_w_icon("you", 7u16),
        Output::Built(vec![
            Element::Const("Hello "),
            Element::String("you".to_string()),
            Element::T(7u16),
        ])
    );
}
