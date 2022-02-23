#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    t.compile_fail("./src/compile_fail/**/*.rs");
    t.pass("./src/compile_pass/**/*.rs");
}
