use javascript_globals::{GLOBALS, GLOBALS_BUILTIN, GLOBALS_BUN};

#[test]
fn test() {
    assert!(!GLOBALS["builtin"]["Date"]);
}

#[test]
fn test_individual_static() {
    assert!(!GLOBALS_BUILTIN["Date"]);
}

#[test]
fn test_bun_environment() {
    assert!(!GLOBALS["bun"]["Bun"]);
    assert!(!GLOBALS_BUN["Bun"]);
}
