use std::fs;

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use serde::Serialize;

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
    let globals_json = fs::read_to_string("node_modules/globals/globals.json")
        .expect("Failed to read node_modules/globals/globals.json. Run `pnpm install` first.");
    let globals: FxHashMap<String, FxHashMap<String, bool>> =
        serde_json::from_str(&globals_json).expect("Failed to parse globals.json");

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
        ("audioworklet", &globals["audioWorklet"]),
        ("browser", &globals["browser"]),
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
        vars: {
            let mut v: Vec<_> = vars
                .iter()
                .map(|(key, value)| EnvVar {
                    name: key,
                    writeable: *value,
                })
                .collect();
            v.sort_by_key(|e| e.name);
            v
        },
    })
    .collect();

    let env_names: Vec<&str> = envs_preset.iter().map(|env| env.name).collect();

    let header = "//! # JavaScript Globals
//!
//! Global identifiers from different JavaScript environments
//!
//! Rust fork of <https://www.npmjs.com/package/globals>";

    // Generate individual statics for each env
    let mut individual_statics = String::new();
    let mut outer_map = phf_codegen::Map::new();

    for env in &envs_preset {
        let static_name = to_static_name(env.name);
        let mut inner_map = phf_codegen::Map::new();
        for var in &env.vars {
            inner_map.entry(var.name, var.writeable.to_string());
        }
        individual_statics.push_str(&format!(
            "#[rustfmt::skip]\npub static {static_name}: phf::Map<&'static str, bool> = {};\n\n",
            inner_map.build()
        ));
        outer_map.entry(env.name, format!("&{static_name}"));
    }

    let out = format!(
        r#"{header}

use core::ops::Index;

/// A map of environment names to their global variable maps.
pub struct Globals(phf::Map<&'static str, &'static phf::Map<&'static str, bool>>);

impl Globals {{
    /// Returns an iterator over the entries of the globals map.
    pub fn entries(
        &self,
    ) -> impl Iterator<Item = (&'static str, &'static phf::Map<&'static str, bool>)> + '_ {{
        self.0.entries().map(|(&k, &v)| (k, v))
    }}

    /// Returns the globals map for the given environment name.
    pub fn get(&self, key: &str) -> Option<&'static phf::Map<&'static str, bool>> {{
        self.0.get(key).copied()
    }}

    /// Returns an iterator over the values of the globals map.
    pub fn values(&self) -> impl Iterator<Item = &'static phf::Map<&'static str, bool>> + '_ {{
        self.0.values().copied()
    }}

    /// Returns true if the globals map contains the given environment name.
    pub fn contains_key(&self, key: &str) -> bool {{
        self.0.contains_key(key)
    }}
}}

impl Index<&str> for Globals {{
    type Output = phf::Map<&'static str, bool>;

    fn index(&self, key: &str) -> &Self::Output {{
        self.0
            .get(key)
            .unwrap_or_else(|| panic!("unknown environment: {{key}}"))
    }}
}}

{individual_statics}#[rustfmt::skip]
pub static GLOBALS: Globals = Globals({globals});
"#,
        globals = outer_map.build()
    );

    fs::write("src/lib.rs", out).unwrap();

    update_readme(&env_names);
}

fn to_static_name(name: &str) -> String {
    format!("GLOBALS_{}", name.to_uppercase().replace('-', "_"))
}

fn update_readme(env_names: &[&str]) {
    let readme = fs::read_to_string("README.md").expect("Failed to read README.md");

    let start_marker = "<!-- GENERATED-ENV-LIST:START - Do not remove or modify this section -->";
    let end_marker = "<!-- GENERATED-ENV-LIST:END -->";

    let start = readme
        .find(start_marker)
        .expect("Could not find start marker in README.md");
    let end = readme
        .find(end_marker)
        .expect("Could not find end marker in README.md");

    let env_list: String = env_names
        .iter()
        .map(|name| format!("- `{name}`\n"))
        .collect();

    let new_readme = format!(
        "{}{start_marker}\n{env_list}{end_marker}{}",
        &readme[..start],
        &readme[end + end_marker.len()..],
    );

    fs::write("README.md", new_readme).unwrap();
}
