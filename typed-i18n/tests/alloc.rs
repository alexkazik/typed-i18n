#![cfg(feature = "alloc")]

use crate::derive::Language;
use std::borrow::Cow;

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
fn string() {
    assert_eq!(Language::En.hello_world::<String>(), "Hello world");
    assert_eq!(Language::De.hello_world::<String>(), "Hallo Welt");

    assert_eq!(
        Language::En.hello_you::<String>("you"),
        "Hello you".to_string()
    );

    assert_eq!(Language::En.maybe_note::<String>(), "");
    assert_eq!(Language::De.maybe_note::<String>(), "Hinweis");
}

#[test]
fn cow() {
    assert_eq!(
        Language::En.hello_world::<Cow<'static, str>>(),
        Cow::Borrowed("Hello world")
    );
    assert_eq!(
        Language::De.hello_world::<Cow<'static, str>>(),
        Cow::Borrowed("Hallo Welt")
    );

    assert_eq!(
        Language::En.hello_you::<Cow<'static, str>>("you"),
        Cow::<'static, str>::Owned("Hello you".to_string())
    );

    assert_eq!(
        Language::En.maybe_note::<Cow<'static, str>>(),
        Cow::Borrowed("")
    );
    assert_eq!(
        Language::De.maybe_note::<Cow<'static, str>>(),
        Cow::Borrowed("Hinweis")
    );
}
