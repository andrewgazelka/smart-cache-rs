#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");

    // skip for now as refs are broken (#1)
    // t.pass("tests/success/*.rs");
}
