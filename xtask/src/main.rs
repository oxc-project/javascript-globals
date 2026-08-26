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
    for entry in fs::read_dir(generated_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "bin") {
            fs::remove_file(path).unwrap();
        }
    }

    let header = "//! # JavaScript Globals
//!
//! Global identifiers from different JavaScript environments
//!
//! Rust fork of <https://www.npmjs.com/package/globals>";

    let global_names = envs_preset
        .iter()
        .flat_map(|env| env.vars.iter().map(|var| var.name))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    assert!(u16::try_from(global_names.len()).is_ok());

    // Use the PHF output position as the stable numeric ID for each name. The runtime map stores
    // offsets into one string, rather than one fat `&str` and one value per occurrence.
    let hash_state = phf_generator::ptrhash::generate_hash(&global_names);
    let mut global_ids = FxHashMap::default();
    let mut global_names_blob = String::new();
    let mut global_name_offsets = Vec::with_capacity(global_names.len() + 1);
    let mut global_name_refs = Vec::with_capacity(global_names.len());

    for (id, &source_index) in hash_state.map.iter().enumerate() {
        let name = global_names[source_index];
        let offset = u16::try_from(global_names_blob.len()).expect("global names exceed 64 KiB");
        global_name_offsets.push(offset);
        global_names_blob.push_str(name);
        global_name_refs.push(name);
        global_ids.insert(name, u16::try_from(id).unwrap());
    }
    global_name_offsets
        .push(u16::try_from(global_names_blob.len()).expect("global names exceed 64 KiB"));

    let mut generated_data = Vec::new();
    let global_name_pilots_start = append_bytes(&mut generated_data, &hash_state.pilots);
    let global_name_remap_start = append_u32s(&mut generated_data, &hash_state.remap);
    let global_names_start = append_bytes(&mut generated_data, global_names_blob.as_bytes());
    let global_name_offsets_start = append_u16s(&mut generated_data, &global_name_offsets);
    let global_name_refs = format_values(&global_name_refs, |value| format!("{value:?}"));
    let global_name_bytes = global_names.len().div_ceil(8);

    // Generate individual statics for each environment. Equal environments share the same static.
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
            let len = env.vars.len();
            individual_statics.push_str(&format!(
                "pub static {static_name}: GlobalSet = GlobalSet::new(\n    &generated_bytes::<0x{members_start:04x}, {global_name_bytes}>(),\n    &generated_u16s::<0x{writable_start:04x}, {}>(),\n    {len},\n);\n\n",
                environment.1.len(),
            ));
            unique_environments.insert(environment, static_name.clone());
        }

        environment_match_arms
            .push_str(&format!("            {:?} => Some(&{static_name}),\n", env.name));
        environment_entries.push_str(&format!("    ({:?}, &{static_name}),\n", env.name));
    }

    fs::write(generated_dir.join("data.bin"), &generated_data).unwrap();
    fs::write(
        generated_dir.join("global_name_refs.rs"),
        format!(
            "#[rustfmt::skip]\nstatic GLOBAL_NAME_REFS: [&str; {}] = [{global_name_refs}\n];\n",
            global_names.len(),
        ),
    )
    .unwrap();
    let generated_data_len = generated_data.len();
    let global_name_pilots_len = hash_state.pilots.len();
    let global_name_remap_len = hash_state.remap.len();
    let global_names_len = global_names_blob.len();
    let global_name_offsets_len = global_name_offsets.len();

    let out = format!(
        r#"{header}

use core::{{
    fmt,
    iter::FusedIterator,
    ops::{{Index, Range}},
}};

const GENERATED_DATA: &[u8; {generated_data_len}] = include_bytes!("generated/data.bin");

const fn generated_bytes<const START: usize, const LEN: usize>() -> [u8; LEN] {{
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {{
        values[index] = GENERATED_DATA[START + index];
        index += 1;
    }}
    values
}}

const fn generated_u16s<const START: usize, const LEN: usize>() -> [u16; LEN] {{
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {{
        let offset = START + index * 2;
        values[index] =
            u16::from_le_bytes([GENERATED_DATA[offset], GENERATED_DATA[offset + 1]]);
        index += 1;
    }}
    values
}}

const fn generated_u32s<const START: usize, const LEN: usize>() -> [u32; LEN] {{
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {{
        let offset = START + index * 4;
        values[index] = u32::from_le_bytes([
            GENERATED_DATA[offset],
            GENERATED_DATA[offset + 1],
            GENERATED_DATA[offset + 2],
            GENERATED_DATA[offset + 3],
        ]);
        index += 1;
    }}
    values
}}

const GLOBAL_NAME_COUNT: usize = {global_name_count};

struct GlobalNames {{
    seed: u64,
    pilots: &'static [u8],
    remap: &'static [u32],
    names: &'static [u8],
    offsets: &'static [u16],
}}

impl GlobalNames {{
    fn get(&self, name: &str) -> Option<u16> {{
        if self.pilots.is_empty() {{
            return None;
        }}
        let hash = phf_shared::ptrhash::hash(name, &self.seed);
        let index = phf_shared::ptrhash::get_index(
            self.seed,
            hash,
            self.pilots,
            self.remap,
            GLOBAL_NAME_COUNT,
        ) as usize;
        (self.name(index) == name.as_bytes()).then_some(index as u16)
    }}

    fn name(&self, id: usize) -> &'static [u8] {{
        // SAFETY: `offsets` is generated with one valid boundary per name plus a final sentinel,
        // and PHF always returns an ID below `GLOBAL_NAME_COUNT`.
        unsafe {{
            let start = usize::from(*self.offsets.get_unchecked(id));
            let end = usize::from(*self.offsets.get_unchecked(id + 1));
            self.names.get_unchecked(start..end)
        }}
    }}
}}

static GLOBAL_NAMES: GlobalNames = GlobalNames {{
    seed: {global_name_seed},
    pilots: &generated_bytes::<0x{global_name_pilots_start:04x}, {global_name_pilots_len}>(),
    remap: &generated_u32s::<0x{global_name_remap_start:04x}, {global_name_remap_len}>(),
    names: &generated_bytes::<0x{global_names_start:04x}, {global_names_len}>(),
    offsets: &generated_u16s::<0x{global_name_offsets_start:04x}, {global_name_offsets_len}>(),
}};

// Retains the previous iterator item types (`&&str`, `&bool`). This table is not referenced by
// lookup-only users such as Oxlint, so the linker can discard it there.
include!("generated/global_name_refs.rs");

/// A compact map of global names to their writability.
#[derive(PartialEq, Eq)]
pub struct GlobalSet {{
    members: &'static [u8; {global_name_bytes}],
    writable: &'static [u16],
    len: u16,
}}

impl GlobalSet {{
    const fn new(members: &'static [u8; {global_name_bytes}], writable: &'static [u16], len: u16) -> Self {{
        Self {{ members, writable, len }}
    }}

    fn global_id(&self, key: &str) -> Option<u16> {{
        let id = GLOBAL_NAMES.get(key)?;
        self.contains_id(id).then_some(id)
    }}

    fn contains_id(&self, id: u16) -> bool {{
        let id = usize::from(id);
        self.members[id / 8] & (1 << (id % 8)) != 0
    }}

    fn is_writable(&self, id: u16) -> bool {{
        self.writable.binary_search(&id).is_ok()
    }}

    /// Returns the number of entries.
    pub const fn len(&self) -> usize {{
        self.len as usize
    }}

    /// Returns whether the map is empty.
    pub const fn is_empty(&self) -> bool {{
        self.len == 0
    }}

    /// Returns whether the map contains `key`.
    pub fn contains_key(&self, key: &str) -> bool {{
        self.global_id(key).is_some()
    }}

    /// Returns the writability of `key`.
    pub fn get(&self, key: &str) -> Option<&'static bool> {{
        let id = self.global_id(key)?;
        Some(if self.is_writable(id) {{ &true }} else {{ &false }})
    }}

    /// Returns the canonical stored key.
    pub fn get_key(&self, key: &str) -> Option<&'static &'static str> {{
        let id = self.global_id(key)?;
        Some(&GLOBAL_NAME_REFS[usize::from(id)])
    }}

    /// Returns the canonical stored key and its writability.
    pub fn get_entry(&self, key: &str) -> Option<(&'static &'static str, &'static bool)> {{
        let id = self.global_id(key)?;
        let name = &GLOBAL_NAME_REFS[usize::from(id)];
        let writable = if self.is_writable(id) {{ &true }} else {{ &false }};
        Some((name, writable))
    }}

    /// Returns an iterator over names and their writability.
    pub fn entries(&self) -> GlobalEntries<'_> {{
        GlobalEntries {{ set: self, ids: 0..GLOBAL_NAME_COUNT, remaining: self.len() }}
    }}

    /// Returns an iterator over names.
    pub fn keys(&self) -> GlobalKeys<'_> {{
        GlobalKeys {{ inner: self.entries() }}
    }}

    /// Returns an iterator over writability values.
    pub fn values(&self) -> GlobalValues<'_> {{
        GlobalValues {{ inner: self.entries() }}
    }}
}}

impl fmt::Debug for GlobalSet {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        f.debug_map().entries(self.entries()).finish()
    }}
}}

impl Index<&str> for GlobalSet {{
    type Output = bool;

    fn index(&self, key: &str) -> &Self::Output {{
        self.get(key).expect("invalid key")
    }}
}}

impl<'a> IntoIterator for &'a GlobalSet {{
    type Item = (&'static &'static str, &'static bool);
    type IntoIter = GlobalEntries<'a>;

    fn into_iter(self) -> Self::IntoIter {{
        self.entries()
    }}
}}

/// Iterator over a [`GlobalSet`]'s entries.
#[derive(Clone)]
pub struct GlobalEntries<'a> {{
    set: &'a GlobalSet,
    ids: Range<usize>,
    remaining: usize,
}}

impl Iterator for GlobalEntries<'_> {{
    type Item = (&'static &'static str, &'static bool);

    fn next(&mut self) -> Option<Self::Item> {{
        while self.ids.start < self.ids.end {{
            let byte_index = self.ids.start / 8;
            let byte = self.set.members[byte_index] & (u8::MAX << (self.ids.start % 8));
            if byte != 0 {{
                let id = byte_index * 8 + byte.trailing_zeros() as usize;
                if id < self.ids.end {{
                    self.ids.start = id + 1;
                    self.remaining -= 1;
                    return Some(global_entry(self.set, id as u16));
                }}
            }}
            self.ids.start = ((byte_index + 1) * 8).min(self.ids.end);
        }}
        None
    }}

    fn size_hint(&self) -> (usize, Option<usize>) {{
        (self.remaining, Some(self.remaining))
    }}
}}

impl DoubleEndedIterator for GlobalEntries<'_> {{
    fn next_back(&mut self) -> Option<Self::Item> {{
        while self.ids.start < self.ids.end {{
            let byte_index = (self.ids.end - 1) / 8;
            let end_bit = (self.ids.end - 1) % 8 + 1;
            let byte = self.set.members[byte_index] & (u8::MAX >> (8 - end_bit));
            if byte != 0 {{
                let id = byte_index * 8 + 7 - byte.leading_zeros() as usize;
                if id >= self.ids.start {{
                    self.ids.end = id;
                    self.remaining -= 1;
                    return Some(global_entry(self.set, id as u16));
                }}
            }}
            self.ids.end = (byte_index * 8).max(self.ids.start);
        }}
        None
    }}
}}

impl ExactSizeIterator for GlobalEntries<'_> {{}}
impl FusedIterator for GlobalEntries<'_> {{}}

fn global_entry(set: &GlobalSet, id: u16) -> (&'static &'static str, &'static bool) {{
    let name = &GLOBAL_NAME_REFS[usize::from(id)];
    let writable = if set.is_writable(id) {{ &true }} else {{ &false }};
    (name, writable)
}}

/// Iterator over a [`GlobalSet`]'s keys.
#[derive(Clone)]
pub struct GlobalKeys<'a> {{
    inner: GlobalEntries<'a>,
}}

impl Iterator for GlobalKeys<'_> {{
    type Item = &'static &'static str;

    fn next(&mut self) -> Option<Self::Item> {{
        self.inner.next().map(|entry| entry.0)
    }}

    fn size_hint(&self) -> (usize, Option<usize>) {{
        self.inner.size_hint()
    }}
}}

impl DoubleEndedIterator for GlobalKeys<'_> {{
    fn next_back(&mut self) -> Option<Self::Item> {{
        self.inner.next_back().map(|entry| entry.0)
    }}
}}

impl ExactSizeIterator for GlobalKeys<'_> {{}}
impl FusedIterator for GlobalKeys<'_> {{}}

/// Iterator over a [`GlobalSet`]'s values.
#[derive(Clone)]
pub struct GlobalValues<'a> {{
    inner: GlobalEntries<'a>,
}}

impl Iterator for GlobalValues<'_> {{
    type Item = &'static bool;

    fn next(&mut self) -> Option<Self::Item> {{
        self.inner.next().map(|entry| entry.1)
    }}

    fn size_hint(&self) -> (usize, Option<usize>) {{
        self.inner.size_hint()
    }}
}}

impl DoubleEndedIterator for GlobalValues<'_> {{
    fn next_back(&mut self) -> Option<Self::Item> {{
        self.inner.next_back().map(|entry| entry.1)
    }}
}}

impl ExactSizeIterator for GlobalValues<'_> {{}}
impl FusedIterator for GlobalValues<'_> {{}}

/// A map of environment names to their global variable maps.
pub struct Globals;

impl Globals {{
    /// Returns an iterator over the entries of the globals map.
    pub fn entries(&self) -> impl Iterator<Item = (&'static str, &'static GlobalSet)> + '_ {{
        ENVIRONMENTS.iter().copied()
    }}

    /// Returns the globals map for the given environment name.
    pub fn get(&self, key: &str) -> Option<&'static GlobalSet> {{
        match key {{
{environment_match_arms}            _ => None,
        }}
    }}

    /// Returns an iterator over the values of the globals map.
    pub fn values(&self) -> impl Iterator<Item = &'static GlobalSet> + '_ {{
        ENVIRONMENTS.iter().map(|entry| entry.1)
    }}

    /// Returns true if the globals map contains the given environment name.
    pub fn contains_key(&self, key: &str) -> bool {{
        self.get(key).is_some()
    }}
}}

impl Index<&str> for Globals {{
    type Output = GlobalSet;

    fn index(&self, key: &str) -> &Self::Output {{
        self.get(key).unwrap_or_else(|| panic!("unknown environment: {{key}}"))
    }}
}}

{individual_statics}static ENVIRONMENTS: [(&str, &GlobalSet); {environment_count}] = [
{environment_entries}];

/// All available environments.
pub static GLOBALS: Globals = Globals;
"#,
        global_name_seed = hash_state.seed,
        global_name_count = global_names.len(),
        global_name_bytes = global_name_bytes,
        environment_count = envs_preset.len(),
    );

    fs::write("src/lib.rs", out).unwrap();

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

fn format_values<T>(values: &[T], format: impl Fn(&T) -> String) -> String {
    values.iter().map(format).fold(String::new(), |mut output, value| {
        output.push_str("\n        ");
        output.push_str(&value);
        output.push(',');
        output
    })
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
