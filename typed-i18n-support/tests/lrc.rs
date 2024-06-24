mod common;

use crate::common::Common;
use typed_i18n_support::diagnostic::Simulated;
use typed_i18n_support::messages::Messages;

#[test]
fn duplicate_key1() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello
# hello
en Hello
"#,
    );
    diagnostic.assert(&["Span: duplicate key hello at line 3"]);
}

#[test]
fn duplicate_key2() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello
# hello
en Hello
# world
en world
"#,
    );
    diagnostic.assert(&["Span: duplicate key hello at line 3"]);
}

#[test]
fn duplicate_language() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
# hello
en Hello
en Hello
"#,
    );
    diagnostic.assert(&["Span: duplicate language en at line 3"]);
}

#[test]
fn value_without_key() {
    let diagnostic = &mut Simulated::new();
    let _messages = Messages::parse(
        diagnostic,
        Common::span(),
        &Common::parameters_lrc(),
        &Common::languages_en_de(),
        r#"
en Error
# hello
en Hello
"#,
    );
    diagnostic.assert(&["Span: value without key at line 1"]);
}
