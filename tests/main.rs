use javascript_globals::GLOBALS;

#[test]
fn test() {
    assert!(!GLOBALS["builtin"]["Date"]);
}
