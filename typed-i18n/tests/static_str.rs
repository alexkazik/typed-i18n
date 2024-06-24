use crate::derive::Language;

mod derive {
    #![no_implicit_prelude]

    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml")]
    #[typed_i18n(builder = "static_str")]
    pub enum Language {
        En,
        De,
    }
}

#[test]
fn basic() {
    assert_eq!(Language::En.hello_world(), "Hello world");
    assert_eq!(Language::De.hello_world(), "Hallo Welt");
}
