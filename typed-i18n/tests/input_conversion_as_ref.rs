use crate::common::{Element, Output};
use crate::derive::Language;
use std::borrow::Cow;

mod common;

mod derive {
    #![no_implicit_prelude]

    use crate::common::Tester;
    use ::std::string::String;
    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "Tester<String>", input = "str", input_conversion = "as_ref")]
    pub enum Language {
        En,
        #[allow(dead_code)]
        De,
    }
}

#[test]
fn input_conversion_as_ref() {
    assert_eq!(
        Language::En.hello_you_w_icon("you", Cow::Owned("🤩".to_string())),
        Output::Built(vec![
            Element::Const("Hello "),
            Element::String("you".to_string()),
            Element::T("🤩".to_string()),
        ])
    );
}
