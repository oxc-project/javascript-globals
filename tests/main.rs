use javascript_globals::{GLOBALS, GLOBALS_BUILTIN, GLOBALS_ES2026};

#[test]
fn test_builtin_environment() {
    assert_eq!(GLOBALS.get("builtin").and_then(|globals| globals.get("Date")), Some(&false));
    assert!(GLOBALS_BUILTIN.contains_key("Date"));
    assert!(GLOBALS_ES2026.contains_key("Iterator"));
}

#[test]
fn test_all_environments_and_iterators() {
    let environments = GLOBALS.entries().collect::<Vec<_>>();
    assert_eq!(environments.len(), 44);

    for (environment_name, globals) in environments {
        assert!(GLOBALS.get(environment_name).is_some_and(|found| core::ptr::eq(found, globals)));

        let entries =
            globals.into_iter().map(|(&name, &writable)| (name, writable)).collect::<Vec<_>>();
        let keys = globals.keys().copied().collect::<Vec<_>>();
        assert_eq!(keys, entries.iter().map(|(name, _)| *name).collect::<Vec<_>>());

        for (name, writable) in entries {
            assert_eq!(globals.get(name).copied(), Some(writable));
            assert!(globals.contains_key(name));
        }
    }

    assert!(GLOBALS.get("not-an-environment").is_none());
    assert!(!GLOBALS_BUILTIN.contains_key("not-a-global"));
    assert!(GLOBALS_BUILTIN.get("not-a-global").is_none());
}

#[test]
fn test_writable_value() {
    assert_eq!(GLOBALS.get("phantomjs").unwrap().get("console"), Some(&true));
}
