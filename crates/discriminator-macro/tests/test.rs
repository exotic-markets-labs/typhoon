#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/**/*.pass.rs");
    t.compile_fail("tests/**/*.fail.rs");
}
