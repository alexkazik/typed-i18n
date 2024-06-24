use typed_i18n_derive::TypedI18N;

#[derive(Copy, Clone, TypedI18N)]
#[typed_i18n(filename = "tests/compile-fail/language_not_known.lrc")]
#[typed_i18n(builder = "mixed_str")]
enum Language {
    En,
}

fn main() {}
