use javascript_globals::{GLOBALS, GLOBALS_BUILTIN};

#[test]
fn test() {
    assert!(!GLOBALS["builtin"]["Date"]);
}

#[test]
fn test_individual_static() {
    assert!(!GLOBALS_BUILTIN["Date"]);
}
