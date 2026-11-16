#![allow(missing_docs)]

// #[rustversion::stable(1.70)] // MSRV
#[test]
fn compile_macros() {
    let t = trybuild::TestCases::new();

    t.pass("tests/trybuild/ok-no-body-type.rs");
    t.pass("tests/trybuild/ok-with-body-type.rs");
    t.pass("tests/trybuild/ok-unrelated-field-attribute.rs");
    t.pass("tests/trybuild/ok-only-copy-from-app-data.rs");

    t.compile_fail("tests/trybuild/err-invalid-structures.rs");
    t.compile_fail("tests/trybuild/err-invalid-from-request-attr.rs");
}
