// Don't run this test on nightly as it depends on the exact output, which changes on nightly a lot.
#[rustversion::not(nightly)]
#[test]
fn trybuild() {
    std::env::set_var(
        "CARGO_MANIFEST_DIR_OVERRIDE",
        std::env::var_os("CARGO_MANIFEST_DIR").unwrap(),
    );
    trybuild::TestCases::new().compile_fail("tests/compile-fail/*.rs");
}
