use typed_i18n_derive::TypedI18N;

#[derive(Copy, Clone, TypedI18N)]
#[typed_i18n(filename = "tests/compile-fail/demo.lrc")]
#[typed_i18n(builder = "static_str")]
#[typed_i18n(unknown = "unknown")]
enum Language {
    En,
}

fn main() {}
