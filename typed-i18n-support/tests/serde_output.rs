use crate::common::Common;
use typed_i18n_support::messages::Messages;

mod common;

#[test]
fn serde_output() {
    let messages = Messages::run_parse(
        &Common::parameters_json(),
        &Common::languages_en_de(),
        r#"{"hello": {"en": "Hello"} }"#,
    )
    .expect("found errors/warnings");
    let flat = serde_json::to_string(&messages).expect("found serde_json errors");
    let tree = serde_json::to_string(&messages.as_tree()).expect("found serde_json errors");
    // the tree is still flat as the source is flat!
    assert_eq!(flat, tree);
}
