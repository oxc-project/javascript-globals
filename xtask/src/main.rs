use std::fs::{self};

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use serde::Serialize;
use ureq::Agent;

const LOCAL_GLOBALS_PATH: &str = "xtask/globals.json";

#[derive(Serialize, Debug)]
struct EnvVar<'a> {
    name: &'a str,
    writeable: bool,
}

#[derive(Serialize, Debug)]
struct Env<'a> {
    name: &'a str,
    vars: Vec<EnvVar<'a>>,
}

fn get_diff(
    current: &FxHashMap<String, bool>,
    prev: &FxHashMap<String, bool>,
) -> FxHashMap<String, bool> {
    let mut retv: FxHashMap<String, bool> = FxHashMap::default();

    for (key, value) in current {
        if !prev.contains_key(key) {
            retv.insert(key.clone(), *value);
        }
    }

    retv
}

lazy_static! {
    static ref NEW_GLOBALS_2017: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("Atomics"), false),
            (String::from("SharedArrayBuffer"), false),
        ]);
    };
    static ref NEW_GLOBALS_2020: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("BigInt"), false),
            (String::from("BigInt64Array"), false),
            (String::from("BigUint64Array"), false),
            (String::from("globalThis"), false),
        ]);
    };
    static ref NEW_GLOBALS_2021: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("AggregateError"), false),
            (String::from("FinalizationRegistry"), false),
            (String::from("WeakRef"), false),
        ]);
    };
    static ref NEW_GLOBALS_2025: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("Float16Array"), false),
            (String::from("Iterator"), false),
        ]);
    };
    // Framework-specific globals
    static ref ASTRO_GLOBALS: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("Astro"), false),
        ]);
    };
    static ref SVELTE_GLOBALS: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("$state"), false),
            (String::from("$derived"), false),
            (String::from("$effect"), false),
            (String::from("$props"), false),
            (String::from("$bindable"), false),
            (String::from("$inspect"), false),
            (String::from("$host"), false),
        ]);
    };
    static ref VUE_GLOBALS: FxHashMap<String, bool> = {
        return FxHashMap::from_iter([
            (String::from("defineProps"), false),
            (String::from("defineEmits"), false),
            (String::from("defineExpose"), false),
            (String::from("withDefaults"), false),
            (String::from("defineOptions"), false),
            (String::from("defineSlots"), false),
            (String::from("defineModel"), false),
        ]);
    };
}

fn main() {
    // Each global is given a value of true or false.
    // A value of true indicates that the variable may be overwritten.
    // A value of false indicates that the variable should be considered read-only.
    // open globals.json file relative to current file
    // let globals: FxHashMap<String, FxHashMap<String, bool>>;
    let globals: FxHashMap<String, FxHashMap<String, bool>> =
        // Try to read from local file first
        if let Ok(file_content) = fs::read_to_string(LOCAL_GLOBALS_PATH) {
            serde_json::from_str(&file_content)
                .expect("Failed to parse local globals.json")
        } else {
            // Fall back to fetching from remote
            match Agent::new_with_defaults()
                .get("https://raw.githubusercontent.com/sindresorhus/globals/main/globals.json")
                .call()
            {
                Ok(mut response) => response
                    .body_mut()
                    .read_json()
                    .expect("Failed to parse globals.json from remote"),
                Err(e) => {
                    panic!("Failed to fetch globals.json: {e}");
                }
            }
        };

    // 19 variables such as Promise, Map, ...
    let new_globals_2015 = get_diff(&globals["es2015"], &globals["es5"]);

    let new_globals_2015_2017 = {
        let mut map = FxHashMap::default();
        map.extend(new_globals_2015.clone());
        map.extend(NEW_GLOBALS_2017.clone());
        map
    };

    let new_globals_2015_2017_2020 = {
        let mut map = new_globals_2015_2017.clone();
        map.extend(NEW_GLOBALS_2020.clone());
        map
    };

    let new_globals_2015_2017_2020_2021 = {
        let mut map = new_globals_2015_2017_2020.clone();
        map.extend(NEW_GLOBALS_2021.clone());
        map
    };

    let new_globals_2015_2017_2020_2021_2025 = {
        let mut map = new_globals_2015_2017_2020_2021.clone();
        map.extend(NEW_GLOBALS_2025.clone());
        map
    };

    let envs_preset: Vec<Env> = [
        // Language
        ("builtin", &globals["builtin"]), // oxc uses builtin instead of es5 of ESLint
        ("es6", &new_globals_2015),
        ("es2015", &new_globals_2015),
        ("es2016", &new_globals_2015),
        ("es2017", &new_globals_2015_2017),
        ("es2018", &new_globals_2015_2017),
        ("es2019", &new_globals_2015_2017),
        ("es2020", &new_globals_2015_2017_2020),
        ("es2021", &new_globals_2015_2017_2020_2021),
        ("es2022", &new_globals_2015_2017_2020_2021),
        ("es2023", &new_globals_2015_2017_2020_2021),
        ("es2024", &new_globals_2015_2017_2020_2021),
        ("es2025", &new_globals_2015_2017_2020_2021_2025),
        ("es2026", &new_globals_2015_2017_2020_2021_2025),
        // Platforms
        ("browser", &globals["browser"]),
        ("audioWorklet", &globals["audioWorklet"]),
        ("node", &globals["node"]),
        ("shared-node-browser", &globals["shared-node-browser"]),
        ("worker", &globals["worker"]),
        ("serviceworker", &globals["serviceworker"]),
        // Frameworks
        ("amd", &globals["amd"]),
        ("applescript", &globals["applescript"]),
        ("astro", &ASTRO_GLOBALS),
        ("atomtest", &globals["atomtest"]),
        ("commonjs", &globals["commonjs"]),
        ("embertest", &globals["embertest"]),
        ("greasemonkey", &globals["greasemonkey"]),
        ("jasmine", &globals["jasmine"]),
        ("jest", &globals["jest"]),
        ("jquery", &globals["jquery"]),
        ("meteor", &globals["meteor"]),
        ("mocha", &globals["mocha"]),
        ("mongo", &globals["mongo"]),
        ("nashorn", &globals["nashorn"]),
        ("protractor", &globals["protractor"]),
        ("prototypejs", &globals["prototypejs"]),
        ("phantomjs", &globals["phantomjs"]),
        ("shelljs", &globals["shelljs"]),
        ("svelte", &SVELTE_GLOBALS),
        ("webextensions", &globals["webextensions"]),
        ("qunit", &globals["qunit"]),
        ("vitest", &globals["vitest"]),
        ("vue", &VUE_GLOBALS),
    ]
    .iter()
    .map(|(name, vars)| Env {
        name,
        vars: vars
            .iter()
            .map(|(key, value)| EnvVar {
                name: key,
                writeable: *value,
            })
            .collect::<Vec<_>>(),
    })
    .collect();

    let mut map = phf_codegen::Map::new();
    for env in envs_preset {
        let mut inner_map = phf_codegen::Map::new();
        for inner_env in env.vars {
            inner_map.entry(inner_env.name, inner_env.writeable.to_string());
        }
        map.entry(env.name, inner_map.build().to_string());
    }

    let header = "//! # JavaScript Globals
//!
//! Global identifiers from different JavaScript environments
//!
//! Rust fork of <https://www.npmjs.com/package/globals>";

    let out = format!(
        "{header}\n\n#[rustfmt::skip]\npub static GLOBALS: phf::Map<&'static str, phf::Map<&'static str, bool>> = {};\n",
        map.build()
    );

    fs::write("src/lib.rs", out).unwrap()
}
