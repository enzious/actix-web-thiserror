#[test]
fn failures() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/compile-fail/*.rs");
}

#[test]
fn parsing_bugs() {
  let t = trybuild::TestCases::new();
  t.pass("tests/bugs/*.rs");
}

/// This is for tests that should compile successfully
/// but are not suitable for examples.
#[test]
fn pass() {
  let t = trybuild::TestCases::new();
  t.pass("tests/pass/*.rs");
}
