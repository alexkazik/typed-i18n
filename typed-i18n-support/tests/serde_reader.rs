mod common;

use crate::common::Common;
use typed_i18n_support::diagnostic::Simulated;
use typed_i18n_support::messages::Messages;

#[test]
fn bad_version() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_json(),
        &Common::languages_en_de(),
        r#"{"_version": 3, "hello": {"en": "Hello"} }"#,
    );
    diagnostic.assert(&["Span: _version is not 2"]);
}

#[test]
fn invalid_json() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_json(),
        &Common::languages_en_de(),
        r#"{"hello": {"en": 4} }"#,
    );
    diagnostic.assert(&["Span: Invalid JSON format, data did not match any variant of untagged enum ObjectOrString at line 1 column 21"]);
}

#[test]
fn invalid_yaml() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_yaml(),
        &Common::languages_en_de(),
        r#"
hello:
  en: 4
"#,
    );
    diagnostic.assert(&["Span: Invalid YAML format, data did not match any variant of untagged enum ObjectOrString at line 2 column 1"]);
}
