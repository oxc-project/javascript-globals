use std::{collections::BTreeSet, fs, path::Path};

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct EnvVar<'a> {
    name: &'a str,
    writable: bool,
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
        ("bun", &globals["bunBuiltin"]),
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
            let mut v: Vec<_> =
                vars.iter().map(|(key, value)| EnvVar { name: key, writable: *value }).collect();
            v.sort_by_key(|e| e.name);
            v
        },
    })
    .collect();

    let env_names: Vec<&str> = envs_preset.iter().map(|env| env.name).collect();

    let generated_dir = Path::new("src/generated");
    fs::create_dir_all(generated_dir).unwrap();

    // All environments share one ID space. Sorting first makes generation deterministic; PHF then
    // permutes these names into its output slots, and each output slot becomes the runtime ID.
    let global_names = envs_preset
        .iter()
        .flat_map(|env| env.vars.iter().map(|var| var.name))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    assert!(!global_names.is_empty());
    assert!(u16::try_from(global_names.len()).is_ok());

    // Store the names in ID order as one UTF-8 blob plus N + 1 offsets. This replaces one fat
    // `&str` and one value for every occurrence of a name in every environment.
    let hash_state = phf_generator::ptrhash::generate_hash(&global_names);
    let mut global_ids = FxHashMap::default();
    let mut global_names_blob = String::new();
    let mut global_name_offsets = Vec::with_capacity(global_names.len() + 1);

    for (id, &source_index) in hash_state.map.iter().enumerate() {
        let name = global_names[source_index];
        let offset = u16::try_from(global_names_blob.len()).expect("global names exceed 64 KiB");
        global_name_offsets.push(offset);
        global_names_blob.push_str(name);
        global_ids.insert(name, u16::try_from(id).unwrap());
    }
    global_name_offsets
        .push(u16::try_from(global_names_blob.len()).expect("global names exceed 64 KiB"));

    // `data.bin` starts with the shared lookup data. Per-environment member bitsets and writable-ID
    // lists are appended below. Every integer is little-endian; generated Rust records the start
    // and length of each section and converts it into typed static arrays during const evaluation.
    let mut generated_data = Vec::new();
    let global_name_pilots_start = append_bytes(&mut generated_data, &hash_state.pilots);
    let global_name_remap_start = append_u32s(&mut generated_data, &hash_state.remap);
    let global_names_start = append_bytes(&mut generated_data, global_names_blob.as_bytes());
    let global_name_offsets_start = append_u16s(&mut generated_data, &global_name_offsets);
    let global_name_bytes = global_names.len().div_ceil(8);

    // Each environment is a fixed-size membership bitset plus a sorted list of writable IDs.
    // Environments with identical pairs alias one canonical static instead of duplicating data.
    let mut individual_statics = String::new();
    let mut unique_environments: FxHashMap<(Vec<u8>, Vec<u16>), String> = FxHashMap::default();
    let mut environment_match_arms = String::new();
    let mut environment_entries = String::new();

    for env in &envs_preset {
        let static_name = to_static_name(env.name);
        let mut members = vec![0_u8; global_name_bytes];
        let mut writable = Vec::new();
        for var in &env.vars {
            let id = global_ids[var.name];
            members[usize::from(id) / 8] |= 1 << (id % 8);
            if var.writable {
                writable.push(id);
            }
        }
        writable.sort_unstable();

        let environment = (members, writable);
        if let Some(canonical_name) = unique_environments.get(&environment) {
            individual_statics.push_str(&format!("pub use {canonical_name} as {static_name};\n\n"));
        } else {
            let members_start = append_bytes(&mut generated_data, &environment.0);
            let writable_start = append_u16s(&mut generated_data, &environment.1);
            individual_statics.push_str(&format!(
                "#[rustfmt::skip]\npub static {static_name}: GlobalSet = GlobalSet {{ members: &bytes::<0x{members_start:04x}, {global_name_bytes}>(), writable: &u16s::<0x{writable_start:04x}, {}>() }};\n\n",
                environment.1.len(),
            ));
            unique_environments.insert(environment, static_name.clone());
        }

        environment_match_arms
            .push_str(&format!("        {:?} => Some(&{static_name}),\n", env.name));
        environment_entries.push_str(&format!("    ({:?}, &{static_name}),\n", env.name));
    }

    fs::write(generated_dir.join("data.bin"), &generated_data).unwrap();
    let generated_data_len = generated_data.len();
    let global_name_pilots_len = hash_state.pilots.len();
    let global_name_remap_len = hash_state.remap.len();
    let global_names_len = global_names_blob.len();
    let global_name_offsets_len = global_name_offsets.len();

    let generated_source = format!(
        r#"// This file is generated by `xtask`.
//
// `data.bin` contains, in order:
// 1. shared ptrhash pilots and remap tables;
// 2. all UTF-8 names concatenated in PHF slot/ID order;
// 3. one u16 offset per name plus a final sentinel;
// 4. a membership bitset and sorted writable-ID list for each unique environment.
//
// The helpers below materialize typed arrays from those byte ranges during const evaluation.
// There is no runtime parsing or allocation.

use super::{{GlobalNames, GlobalSet, Globals}};

const DATA: &[u8; {generated_data_len}] = include_bytes!("data.bin");

const fn bytes<const START: usize, const LEN: usize>() -> [u8; LEN] {{
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {{
        values[index] = DATA[START + index];
        index += 1;
    }}
    values
}}

const fn u16s<const START: usize, const LEN: usize>() -> [u16; LEN] {{
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {{
        let offset = START + index * 2;
        values[index] = u16::from_le_bytes([DATA[offset], DATA[offset + 1]]);
        index += 1;
    }}
    values
}}

const fn u32s<const START: usize, const LEN: usize>() -> [u32; LEN] {{
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {{
        let offset = START + index * 4;
        values[index] = u32::from_le_bytes([
            DATA[offset],
            DATA[offset + 1],
            DATA[offset + 2],
            DATA[offset + 3],
        ]);
        index += 1;
    }}
    values
}}

// Preserve the map-compatible iterator item type without emitting one Rust string literal per
// name. Every reference points into the single name blob owned by `GLOBAL_NAMES`.
const fn global_name_refs(names: &'static [u8], offsets: &'static [u16]) -> [&'static str; {global_name_count}] {{
    let mut refs = [""; {global_name_count}];
    let mut index = 0;
    while index < refs.len() {{
        let start = offsets[index] as usize;
        let end = offsets[index + 1] as usize;
        // SAFETY: The generator concatenates valid UTF-8 strings and records their boundaries.
        refs[index] = unsafe {{
            let bytes = core::slice::from_raw_parts(names.as_ptr().add(start), end - start);
            core::str::from_utf8_unchecked(bytes)
        }};
        index += 1;
    }}
    refs
}}

pub(super) const GLOBAL_NAME_COUNT: usize = {global_name_count};
pub(super) const GLOBAL_NAME_BYTES: usize = {global_name_bytes};

// A ptrhash output slot is the global's shared numeric ID. `GlobalNames::get` verifies the string
// at the candidate slot because a perfect hash only distinguishes keys from its generated set.
pub(super) static GLOBAL_NAMES: GlobalNames = GlobalNames {{
    seed: {global_name_seed},
    pilots: &bytes::<0x{global_name_pilots_start:04x}, {global_name_pilots_len}>(),
    remap: &u32s::<0x{global_name_remap_start:04x}, {global_name_remap_len}>(),
    names: &bytes::<0x{global_names_start:04x}, {global_names_len}>(),
    offsets: &u16s::<0x{global_name_offsets_start:04x}, {global_name_offsets_len}>(),
}};

pub(super) static GLOBAL_NAME_REFS: [&str; {global_name_count}] =
    global_name_refs(GLOBAL_NAMES.names, GLOBAL_NAMES.offsets);

// Each set references its member bitset and sorted writable-ID slice inside `data.bin`. Equal sets
// are emitted as aliases by the generator.
{individual_statics}pub(super) static ENVIRONMENTS: [(&str, &GlobalSet); {environment_count}] = [
{environment_entries}];

// Keep iteration data separate from lookup code: users of `GLOBALS.get` get a direct match, and
// linkers can discard `ENVIRONMENTS` when `GLOBALS.entries` is unused.
pub(super) fn get_environment(key: &str) -> Option<&'static GlobalSet> {{
    match key {{
{environment_match_arms}        _ => None,
    }}
}}

/// All available environments.
pub static GLOBALS: Globals = Globals;
"#,
        global_name_seed = hash_state.seed,
        global_name_count = global_names.len(),
        environment_count = envs_preset.len(),
    );

    fs::write(generated_dir.join("mod.rs"), generated_source).unwrap();

    update_readme(&env_names);
}

fn to_static_name(name: &str) -> String {
    format!("GLOBALS_{}", name.to_uppercase().replace('-', "_"))
}

fn append_bytes(data: &mut Vec<u8>, values: &[u8]) -> usize {
    let start = data.len();
    data.extend_from_slice(values);
    start
}

fn append_u16s(data: &mut Vec<u8>, values: &[u16]) -> usize {
    let start = data.len();
    data.extend(values.iter().flat_map(|value| value.to_le_bytes()));
    start
}

fn append_u32s(data: &mut Vec<u8>, values: &[u32]) -> usize {
    let start = data.len();
    data.extend(values.iter().flat_map(|value| value.to_le_bytes()));
    start
}

fn update_readme(env_names: &[&str]) {
    let readme = fs::read_to_string("README.md").expect("Failed to read README.md");

    let start_marker = "<!-- GENERATED-ENV-LIST:START - Do not remove or modify this section -->";
    let end_marker = "<!-- GENERATED-ENV-LIST:END -->";

    let start = readme.find(start_marker).expect("Could not find start marker in README.md");
    let end = readme.find(end_marker).expect("Could not find end marker in README.md");

    let env_list: String = env_names.iter().map(|name| format!("- `{name}`\n")).collect();

    // Surround the list with blank lines so the output matches `dprint fmt`'s
    // markdown formatting; otherwise xtask and dprint fight over README.md.
    let new_readme = format!(
        "{}{start_marker}\n\n{env_list}\n{end_marker}{}",
        &readme[..start],
        &readme[end + end_marker.len()..],
    );

    fs::write("README.md", new_readme).unwrap();
}
