use typed_i18n_derive::TypedI18N;

#[derive(Copy, Clone, TypedI18N)]
#[typed_i18n(filename = "tests/compile-fail/demo.lrc")]
enum Language {
    En,
}

fn main() {}
