#[test]
fn trybuild() {
    // Don't run this test on nightly as it depends on the exact output, which changes on nightly a lot.
    if !env!("RUSTUP_TOOLCHAIN").starts_with("nightly") {
        // Safety: The environment access only happens in single-threaded code.
        unsafe {
            std::env::set_var(
                "CARGO_MANIFEST_DIR_OVERRIDE",
                std::env::var_os("CARGO_MANIFEST_DIR").unwrap(),
            )
        };
        trybuild::TestCases::new().compile_fail("tests/compile-fail/*.rs");
    }
}
