use crate::derive::Language;

mod derive {
    #![no_implicit_prelude]

    use ::typed_i18n::TypedI18N;

    #[derive(Copy, Clone, TypedI18N)]
    #[typed_i18n(filename = "example.yaml", global = "atomic")]
    #[typed_i18n(builder = "static_str")]
    pub enum Language {
        En,
        #[typed_i18n(default = "true")]
        De,
    }
}

#[test]
fn global_atomic() {
    // de is the default
    assert_eq!(Language::global().hello_world(), "Hallo Welt");
    Language::En.set_global();
    assert_eq!(Language::global().hello_world(), "Hello world");
}
