mod common;

use crate::common::Common;
use typed_i18n_support::diagnostic::Simulated;
use typed_i18n_support::languages::{Language, RawLanguages};

// all this tests are independent of the format

#[test]
fn no_languages() {
    let diagnostic = &mut Simulated::new();
    let _ = RawLanguages(vec![]).into(diagnostic, Common::span());
    diagnostic.assert(&["Span: no languages found"]);
}

#[test]
fn duplicate_languages() {
    let diagnostic = &mut Simulated::new();
    let _ = RawLanguages(vec![
        Language::run_new("en", &[]),
        Language::run_new("en", &[]),
    ])
    .into(diagnostic, Common::span());
    diagnostic.assert(&["Span: language defined twice"]);
}

#[test]
fn fallback_itself() {
    let diagnostic = &mut Simulated::new();
    let _ = RawLanguages(vec![Language::run_new("en", &["en"])]).into(diagnostic, Common::span());
    diagnostic.assert(&["Span: fallback of itself"]);
}

#[test]
fn unknown_fallback() {
    let diagnostic = &mut Simulated::new();
    let _ =
        RawLanguages(vec![Language::run_new("en", &["alien"])]).into(diagnostic, Common::span());
    diagnostic.assert(&["Span: unknown fallback: alien"]);
}
