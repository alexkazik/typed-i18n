mod common;

use crate::common::Common;
use typed_i18n_support::diagnostic::Simulated;
use typed_i18n_support::messages::Messages;

// all this tests are independent of the format

#[test]
fn no_messages() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_json(),
        &Common::languages_en_de(),
        r#"{}"#,
    );
    diagnostic.assert_errors(&["Span: no messages found"]);
}

#[test]
fn language_warning() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_json(),
        &Common::languages_en_de(),
        r#"{"hello": {"en": "Hello", "unk": "Hello"} }"#,
    );
    diagnostic.assert_warnings(&["Span: language unk key hello is not known"]);
}
