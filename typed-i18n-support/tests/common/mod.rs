#![allow(dead_code)]

use proc_macro2::Span;
use typed_i18n_support::attribute::Parameters;
use typed_i18n_support::languages::{Language, Languages, RawLanguages};

pub struct Common;

impl Common {
    pub fn span() -> Span {
        Span::call_site()
    }

    pub fn languages_en_de() -> Languages {
        RawLanguages(vec![
            Language::run_new("en", &[]),
            Language::run_new("de", &[]),
        ])
        .run_into()
        .expect("languages")
    }

    pub fn parameters_lrc() -> Parameters {
        Parameters::run_new("_.lrc", "_")
    }

    pub fn parameters_json() -> Parameters {
        Parameters::run_new("_.json", "_")
    }

    pub fn parameters_yaml() -> Parameters {
        Parameters::run_new("_.yaml", "_")
    }
}
