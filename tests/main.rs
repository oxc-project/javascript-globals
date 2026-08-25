use javascript_globals::{
    GLOBALS, GLOBALS_BUILTIN, GLOBALS_BUN, GLOBALS_ES6, GLOBALS_ES2015, GLOBALS_ES2016,
    GLOBALS_ES2017, GLOBALS_ES2018, GLOBALS_ES2019, GLOBALS_ES2021, GLOBALS_ES2022, GLOBALS_ES2023,
    GLOBALS_ES2024, GLOBALS_ES2025, GLOBALS_ES2026, GLOBALS_PHANTOMJS,
};

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

#[test]
fn test_all_environments_and_iterators() {
    let environments = GLOBALS.entries().collect::<Vec<_>>();
    assert_eq!(environments.len(), 44);

    for (environment_name, globals) in environments {
        assert_eq!(GLOBALS.get(environment_name), Some(globals));
        assert!(!globals.is_empty());
        assert_eq!(globals.entries().len(), globals.len());
        assert_eq!(globals.keys().len(), globals.len());
        assert_eq!(globals.values().len(), globals.len());

        let forward =
            globals.entries().map(|(&name, &writable)| (name, writable)).collect::<Vec<_>>();
        let backward =
            globals.entries().rev().map(|(&name, &writable)| (name, writable)).collect::<Vec<_>>();
        assert_eq!(backward, forward.iter().rev().copied().collect::<Vec<_>>());

        let mut mixed = globals.entries();
        assert_eq!(
            mixed.next().map(|(&name, &writable)| (name, writable)),
            forward.first().copied()
        );
        if globals.len() > 1 {
            assert_eq!(
                mixed.next_back().map(|(&name, &writable)| (name, writable)),
                forward.last().copied()
            );
            assert_eq!(mixed.len(), globals.len() - 2);
        }

        for (&name, &writable) in globals {
            assert_eq!(globals.get(name).copied(), Some(writable));
            assert_eq!(globals.get_key(name).copied(), Some(name));
            assert_eq!(
                globals
                    .get_entry(name)
                    .map(|(stored_name, stored_writable)| (*stored_name, *stored_writable)),
                Some((name, writable))
            );
            assert!(globals.contains_key(name));
        }
    }

    assert!(GLOBALS.get("not-an-environment").is_none());
    assert!(!GLOBALS_BUILTIN.contains_key("not-a-global"));
    assert!(GLOBALS_BUILTIN.get("not-a-global").is_none());
}

#[test]
fn test_writable_value() {
    assert_eq!(GLOBALS_PHANTOMJS.get("console"), Some(&true));
}

#[test]
fn test_equivalent_ecmascript_environments_are_aliases() {
    assert!(core::ptr::eq(&GLOBALS_ES6, &GLOBALS_ES2015));
    assert!(core::ptr::eq(&GLOBALS_ES6, &GLOBALS_ES2016));
    assert!(core::ptr::eq(&GLOBALS_ES2017, &GLOBALS_ES2018));
    assert!(core::ptr::eq(&GLOBALS_ES2017, &GLOBALS_ES2019));
    assert!(core::ptr::eq(&GLOBALS_ES2021, &GLOBALS_ES2022));
    assert!(core::ptr::eq(&GLOBALS_ES2021, &GLOBALS_ES2023));
    assert!(core::ptr::eq(&GLOBALS_ES2021, &GLOBALS_ES2024));
    assert!(core::ptr::eq(&GLOBALS_ES2025, &GLOBALS_ES2026));
}
