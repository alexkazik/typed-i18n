use crate::common::{Element, Output};
use crate::derive::Language;

mod common;

mod derive {
    #![no_implicit_prelude]

    use crate::common::Tester;
    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "Tester<bool>", input = "bool")]
    pub enum Language {
        En,
        De,
    }
}

#[test]
fn basic() {
    assert_eq!(Language::En.hello_world(), Output::Const("Hello world"));
    assert_eq!(Language::De.hello_world(), Output::Const("Hallo Welt"));

    assert_eq!(
        Language::En.hello_you("you"),
        Output::Built(vec![
            Element::Const("Hello "),
            Element::String("you".to_string()),
        ])
    );

    assert_eq!(
        Language::En.hello_you_w_icon("you", false),
        Output::Built(vec![
            Element::Const("Hello "),
            Element::String("you".to_string()),
            Element::T(false),
        ])
    );

    assert_eq!(Language::En.maybe_note(), Output::Empty);
    assert_eq!(Language::De.maybe_note(), Output::Const("Hinweis"));
}
