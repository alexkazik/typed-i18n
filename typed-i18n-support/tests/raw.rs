mod common;

use crate::common::Common;
use typed_i18n_support::attribute::Parameters;
use typed_i18n_support::diagnostic::Simulated;
use typed_i18n_support::messages::Messages;

// all this tests are independent of the format

#[test]
fn no_extension() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Parameters::run_new("_", "_"),
        &Common::languages_en_de(),
        "",
    );
    diagnostic.assert(&["Span: No file extension on \"_\""]);
}

#[test]
fn unknown_extension() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Parameters::run_new("_._", "_"),
        &Common::languages_en_de(),
        "",
    );
    diagnostic.assert(&["Span: Unsupported file extension \"_\""]);
}

#[test]
fn no_messages() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#""#,
    );
    diagnostic.assert(&["Span: no messages found"]);
}

#[test]
fn language_warning() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello
unk Hello
"#,
    );
    diagnostic.assert(&["Span: language unk is in the file (key hello) but not the enum"]);
}

#[test]
fn language_ignored() {
    let mut parameters = Common::parameters_lrc();
    parameters.ignore_languages = vec!["unk".into()];
    Messages::run_parse(
        &parameters,
        &Common::languages_en_de(),
        r#"
# hello
en Hello
unk Hello
"#,
    )
    .expect("found errors/warnings");
}

#[test]
fn language_ignored_all() {
    let mut parameters = Common::parameters_lrc();
    parameters.ignore_languages = vec!["*".into()];
    Messages::run_parse(
        &parameters,
        &Common::languages_en_de(),
        r#"
# hello
en Hello
unk Hello
"#,
    )
    .expect("found errors/warnings");
}

#[test]
fn language_warning_no_windcard() {
    let diagnostic = &mut Simulated::new();
    let mut parameters = Common::parameters_lrc();
    parameters.ignore_languages = vec!["u*".into()];
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &parameters,
        &Common::languages_en_de(),
        r#"
# hello
en Hello
unk Hello
"#,
    );
    diagnostic.assert(&["Span: language unk is in the file (key hello) but not the enum"]);
}

#[test]
fn mismatching_types() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello %{name}
de Hallo *{name}
"#,
    );
    diagnostic.assert(&["Span: mismatching types for parameter name in key de"]);
}

#[test]
fn duplicate_typed_parameter() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello *{name}/*{name}
"#,
    );
    diagnostic.assert(&["Span: duplicate use of a typed parameter: \"name\" in key hello.en"]);
}

#[test]
fn key_without_values() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello
# world
"#,
    );
    diagnostic.assert(&["Span: key world has no values"]);
}

#[test]
fn parse_error1() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello %{
"#,
    );
    diagnostic.assert(&["Span: parse error in hello.en"]);
}

#[test]
fn parse_error2() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello %{ %{
"#,
    );
    diagnostic.assert(&["Span: parse error in hello.en"]);
}
