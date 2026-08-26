//! # JavaScript Globals
//!
//! Global identifiers from different JavaScript environments
//!
//! Rust fork of <https://www.npmjs.com/package/globals>

use core::{
    fmt,
    iter::FusedIterator,
    ops::{Index, Range},
};

const GENERATED_DATA: &[u8; 33711] = include_bytes!("generated/data.bin");

const fn generated_bytes<const START: usize, const LEN: usize>() -> [u8; LEN] {
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {
        values[index] = GENERATED_DATA[START + index];
        index += 1;
    }
    values
}

const fn generated_u16s<const START: usize, const LEN: usize>() -> [u16; LEN] {
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {
        let offset = START + index * 2;
        values[index] = u16::from_le_bytes([GENERATED_DATA[offset], GENERATED_DATA[offset + 1]]);
        index += 1;
    }
    values
}

const fn generated_u32s<const START: usize, const LEN: usize>() -> [u32; LEN] {
    let mut values = [0; LEN];
    let mut index = 0;
    while index < LEN {
        let offset = START + index * 4;
        values[index] = u32::from_le_bytes([
            GENERATED_DATA[offset],
            GENERATED_DATA[offset + 1],
            GENERATED_DATA[offset + 2],
            GENERATED_DATA[offset + 3],
        ]);
        index += 1;
    }
    values
}

const GLOBAL_NAME_COUNT: usize = 1638;

struct GlobalNames {
    seed: u64,
    pilots: &'static [u8],
    remap: &'static [u32],
    names: &'static [u8],
    offsets: &'static [u16],
}

impl GlobalNames {
    fn get(&self, name: &str) -> Option<u16> {
        if self.pilots.is_empty() {
            return None;
        }
        let hash = phf_shared::ptrhash::hash(name, &self.seed);
        let index = phf_shared::ptrhash::get_index(
            self.seed,
            hash,
            self.pilots,
            self.remap,
            GLOBAL_NAME_COUNT,
        ) as usize;
        (self.name(index) == name.as_bytes()).then_some(index as u16)
    }

    fn name(&self, id: usize) -> &'static [u8] {
        // SAFETY: `offsets` is generated with one valid boundary per name plus a final sentinel,
        // and PHF always returns an ID below `GLOBAL_NAME_COUNT`.
        unsafe {
            let start = usize::from(*self.offsets.get_unchecked(id));
            let end = usize::from(*self.offsets.get_unchecked(id + 1));
            self.names.get_unchecked(start..end)
        }
    }
}

static GLOBAL_NAMES: GlobalNames = GlobalNames {
    seed: 16287231350648472473,
    pilots: &generated_bytes::<0x0000, 549>(),
    remap: &generated_u32s::<0x0225, 17>(),
    names: &generated_bytes::<0x0269, 22100>(),
    offsets: &generated_u16s::<0x58bd, 1639>(),
};

// Retains the previous iterator item types (`&&str`, `&bool`). This table is not referenced by
// lookup-only users such as Oxlint, so the linker can discard it there.
include!("generated/global_name_refs.rs");

/// A compact map of global names to their writability.
#[derive(PartialEq, Eq)]
pub struct GlobalSet {
    members: &'static [u8; 205],
    writable: &'static [u16],
    len: u16,
}

impl GlobalSet {
    const fn new(members: &'static [u8; 205], writable: &'static [u16], len: u16) -> Self {
        Self { members, writable, len }
    }

    fn global_id(&self, key: &str) -> Option<u16> {
        let id = GLOBAL_NAMES.get(key)?;
        self.contains_id(id).then_some(id)
    }

    fn contains_id(&self, id: u16) -> bool {
        let id = usize::from(id);
        self.members[id / 8] & (1 << (id % 8)) != 0
    }

    fn is_writable(&self, id: u16) -> bool {
        self.writable.binary_search(&id).is_ok()
    }

    /// Returns the number of entries.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns whether the map is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns whether the map contains `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.global_id(key).is_some()
    }

    /// Returns the writability of `key`.
    pub fn get(&self, key: &str) -> Option<&'static bool> {
        let id = self.global_id(key)?;
        Some(if self.is_writable(id) { &true } else { &false })
    }

    /// Returns the canonical stored key.
    pub fn get_key(&self, key: &str) -> Option<&'static &'static str> {
        let id = self.global_id(key)?;
        Some(&GLOBAL_NAME_REFS[usize::from(id)])
    }

    /// Returns the canonical stored key and its writability.
    pub fn get_entry(&self, key: &str) -> Option<(&'static &'static str, &'static bool)> {
        let id = self.global_id(key)?;
        let name = &GLOBAL_NAME_REFS[usize::from(id)];
        let writable = if self.is_writable(id) { &true } else { &false };
        Some((name, writable))
    }

    /// Returns an iterator over names and their writability.
    pub fn entries(&self) -> GlobalEntries<'_> {
        GlobalEntries { set: self, ids: 0..GLOBAL_NAME_COUNT, remaining: self.len() }
    }

    /// Returns an iterator over names.
    pub fn keys(&self) -> GlobalKeys<'_> {
        GlobalKeys { inner: self.entries() }
    }

    /// Returns an iterator over writability values.
    pub fn values(&self) -> GlobalValues<'_> {
        GlobalValues { inner: self.entries() }
    }
}

impl fmt::Debug for GlobalSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.entries()).finish()
    }
}

impl Index<&str> for GlobalSet {
    type Output = bool;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).expect("invalid key")
    }
}

impl<'a> IntoIterator for &'a GlobalSet {
    type Item = (&'static &'static str, &'static bool);
    type IntoIter = GlobalEntries<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries()
    }
}

/// Iterator over a [`GlobalSet`]'s entries.
#[derive(Clone)]
pub struct GlobalEntries<'a> {
    set: &'a GlobalSet,
    ids: Range<usize>,
    remaining: usize,
}

impl Iterator for GlobalEntries<'_> {
    type Item = (&'static &'static str, &'static bool);

    fn next(&mut self) -> Option<Self::Item> {
        while self.ids.start < self.ids.end {
            let byte_index = self.ids.start / 8;
            let byte = self.set.members[byte_index] & (u8::MAX << (self.ids.start % 8));
            if byte != 0 {
                let id = byte_index * 8 + byte.trailing_zeros() as usize;
                if id < self.ids.end {
                    self.ids.start = id + 1;
                    self.remaining -= 1;
                    return Some(global_entry(self.set, id as u16));
                }
            }
            self.ids.start = ((byte_index + 1) * 8).min(self.ids.end);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl DoubleEndedIterator for GlobalEntries<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.ids.start < self.ids.end {
            let byte_index = (self.ids.end - 1) / 8;
            let end_bit = (self.ids.end - 1) % 8 + 1;
            let byte = self.set.members[byte_index] & (u8::MAX >> (8 - end_bit));
            if byte != 0 {
                let id = byte_index * 8 + 7 - byte.leading_zeros() as usize;
                if id >= self.ids.start {
                    self.ids.end = id;
                    self.remaining -= 1;
                    return Some(global_entry(self.set, id as u16));
                }
            }
            self.ids.end = (byte_index * 8).max(self.ids.start);
        }
        None
    }
}

impl ExactSizeIterator for GlobalEntries<'_> {}
impl FusedIterator for GlobalEntries<'_> {}

fn global_entry(set: &GlobalSet, id: u16) -> (&'static &'static str, &'static bool) {
    let name = &GLOBAL_NAME_REFS[usize::from(id)];
    let writable = if set.is_writable(id) { &true } else { &false };
    (name, writable)
}

/// Iterator over a [`GlobalSet`]'s keys.
#[derive(Clone)]
pub struct GlobalKeys<'a> {
    inner: GlobalEntries<'a>,
}

impl Iterator for GlobalKeys<'_> {
    type Item = &'static &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for GlobalKeys<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|entry| entry.0)
    }
}

impl ExactSizeIterator for GlobalKeys<'_> {}
impl FusedIterator for GlobalKeys<'_> {}

/// Iterator over a [`GlobalSet`]'s values.
#[derive(Clone)]
pub struct GlobalValues<'a> {
    inner: GlobalEntries<'a>,
}

impl Iterator for GlobalValues<'_> {
    type Item = &'static bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry.1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for GlobalValues<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|entry| entry.1)
    }
}

impl ExactSizeIterator for GlobalValues<'_> {}
impl FusedIterator for GlobalValues<'_> {}

/// A map of environment names to their global variable maps.
pub struct Globals;

impl Globals {
    /// Returns an iterator over the entries of the globals map.
    pub fn entries(&self) -> impl Iterator<Item = (&'static str, &'static GlobalSet)> + '_ {
        ENVIRONMENTS.iter().copied()
    }

    /// Returns the globals map for the given environment name.
    pub fn get(&self, key: &str) -> Option<&'static GlobalSet> {
        match key {
            "builtin" => Some(&GLOBALS_BUILTIN),
            "es6" => Some(&GLOBALS_ES6),
            "es2015" => Some(&GLOBALS_ES2015),
            "es2016" => Some(&GLOBALS_ES2016),
            "es2017" => Some(&GLOBALS_ES2017),
            "es2018" => Some(&GLOBALS_ES2018),
            "es2019" => Some(&GLOBALS_ES2019),
            "es2020" => Some(&GLOBALS_ES2020),
            "es2021" => Some(&GLOBALS_ES2021),
            "es2022" => Some(&GLOBALS_ES2022),
            "es2023" => Some(&GLOBALS_ES2023),
            "es2024" => Some(&GLOBALS_ES2024),
            "es2025" => Some(&GLOBALS_ES2025),
            "es2026" => Some(&GLOBALS_ES2026),
            "audioworklet" => Some(&GLOBALS_AUDIOWORKLET),
            "browser" => Some(&GLOBALS_BROWSER),
            "bun" => Some(&GLOBALS_BUN),
            "node" => Some(&GLOBALS_NODE),
            "shared-node-browser" => Some(&GLOBALS_SHARED_NODE_BROWSER),
            "worker" => Some(&GLOBALS_WORKER),
            "serviceworker" => Some(&GLOBALS_SERVICEWORKER),
            "amd" => Some(&GLOBALS_AMD),
            "applescript" => Some(&GLOBALS_APPLESCRIPT),
            "astro" => Some(&GLOBALS_ASTRO),
            "atomtest" => Some(&GLOBALS_ATOMTEST),
            "commonjs" => Some(&GLOBALS_COMMONJS),
            "embertest" => Some(&GLOBALS_EMBERTEST),
            "greasemonkey" => Some(&GLOBALS_GREASEMONKEY),
            "jasmine" => Some(&GLOBALS_JASMINE),
            "jest" => Some(&GLOBALS_JEST),
            "jquery" => Some(&GLOBALS_JQUERY),
            "meteor" => Some(&GLOBALS_METEOR),
            "mocha" => Some(&GLOBALS_MOCHA),
            "mongo" => Some(&GLOBALS_MONGO),
            "nashorn" => Some(&GLOBALS_NASHORN),
            "protractor" => Some(&GLOBALS_PROTRACTOR),
            "prototypejs" => Some(&GLOBALS_PROTOTYPEJS),
            "phantomjs" => Some(&GLOBALS_PHANTOMJS),
            "shelljs" => Some(&GLOBALS_SHELLJS),
            "svelte" => Some(&GLOBALS_SVELTE),
            "webextensions" => Some(&GLOBALS_WEBEXTENSIONS),
            "qunit" => Some(&GLOBALS_QUNIT),
            "vitest" => Some(&GLOBALS_VITEST),
            "vue" => Some(&GLOBALS_VUE),
            _ => None,
        }
    }

    /// Returns an iterator over the values of the globals map.
    pub fn values(&self) -> impl Iterator<Item = &'static GlobalSet> + '_ {
        ENVIRONMENTS.iter().map(|entry| entry.1)
    }

    /// Returns true if the globals map contains the given environment name.
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

impl Index<&str> for Globals {
    type Output = GlobalSet;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).unwrap_or_else(|| panic!("unknown environment: {key}"))
    }
}

pub static GLOBALS_BUILTIN: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x658b, 205>(), &generated_u16s::<0x6658, 0>(), 65);

pub static GLOBALS_ES6: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6658, 205>(), &generated_u16s::<0x6725, 0>(), 20);

pub use GLOBALS_ES6 as GLOBALS_ES2015;

pub use GLOBALS_ES6 as GLOBALS_ES2016;

pub static GLOBALS_ES2017: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6725, 205>(), &generated_u16s::<0x67f2, 0>(), 22);

pub use GLOBALS_ES2017 as GLOBALS_ES2018;

pub use GLOBALS_ES2017 as GLOBALS_ES2019;

pub static GLOBALS_ES2020: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x67f2, 205>(), &generated_u16s::<0x68bf, 0>(), 26);

pub static GLOBALS_ES2021: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x68bf, 205>(), &generated_u16s::<0x698c, 0>(), 29);

pub use GLOBALS_ES2021 as GLOBALS_ES2022;

pub use GLOBALS_ES2021 as GLOBALS_ES2023;

pub use GLOBALS_ES2021 as GLOBALS_ES2024;

pub static GLOBALS_ES2025: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x698c, 205>(), &generated_u16s::<0x6a59, 0>(), 31);

pub use GLOBALS_ES2025 as GLOBALS_ES2026;

pub static GLOBALS_AUDIOWORKLET: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6a59, 205>(), &generated_u16s::<0x6b26, 0>(), 54);

pub static GLOBALS_BROWSER: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6b26, 205>(), &generated_u16s::<0x6bf3, 125>(), 1196);

pub static GLOBALS_BUN: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6ced, 205>(), &generated_u16s::<0x6dba, 2>(), 92);

pub static GLOBALS_NODE: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6dbe, 205>(), &generated_u16s::<0x6e8b, 1>(), 81);

pub static GLOBALS_SHARED_NODE_BROWSER: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6e8d, 205>(), &generated_u16s::<0x6f5a, 0>(), 71);

pub static GLOBALS_WORKER: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x6f5a, 205>(), &generated_u16s::<0x7027, 9>(), 347);

pub static GLOBALS_SERVICEWORKER: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7039, 205>(), &generated_u16s::<0x7106, 25>(), 326);

pub static GLOBALS_AMD: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7138, 205>(), &generated_u16s::<0x7205, 0>(), 2);

pub static GLOBALS_APPLESCRIPT: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7205, 205>(), &generated_u16s::<0x72d2, 0>(), 11);

pub static GLOBALS_ASTRO: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x72d2, 205>(), &generated_u16s::<0x739f, 0>(), 1);

pub static GLOBALS_ATOMTEST: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x739f, 205>(), &generated_u16s::<0x746c, 0>(), 8);

pub static GLOBALS_COMMONJS: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x746c, 205>(), &generated_u16s::<0x7539, 1>(), 4);

pub static GLOBALS_EMBERTEST: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x753b, 205>(), &generated_u16s::<0x7608, 0>(), 15);

pub static GLOBALS_GREASEMONKEY: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7608, 205>(), &generated_u16s::<0x76d5, 0>(), 31);

pub static GLOBALS_JASMINE: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x76d5, 205>(), &generated_u16s::<0x77a2, 0>(), 23);

pub static GLOBALS_JEST: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x77a2, 205>(), &generated_u16s::<0x786f, 0>(), 13);

pub static GLOBALS_JQUERY: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x786f, 205>(), &generated_u16s::<0x793c, 0>(), 2);

pub static GLOBALS_METEOR: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x793c, 205>(), &generated_u16s::<0x7a09, 0>(), 41);

pub static GLOBALS_MOCHA: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7a09, 205>(), &generated_u16s::<0x7ad6, 0>(), 20);

pub static GLOBALS_MONGO: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7ad6, 205>(), &generated_u16s::<0x7ba3, 0>(), 31);

pub static GLOBALS_NASHORN: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7ba3, 205>(), &generated_u16s::<0x7c70, 0>(), 18);

pub static GLOBALS_PROTRACTOR: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7c70, 205>(), &generated_u16s::<0x7d3d, 0>(), 8);

pub static GLOBALS_PROTOTYPEJS: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7d3d, 205>(), &generated_u16s::<0x7e0a, 0>(), 38);

pub static GLOBALS_PHANTOMJS: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7e0a, 205>(), &generated_u16s::<0x7ed7, 5>(), 5);

pub static GLOBALS_SHELLJS: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7ee1, 205>(), &generated_u16s::<0x7fae, 0>(), 34);

pub static GLOBALS_SVELTE: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x7fae, 205>(), &generated_u16s::<0x807b, 0>(), 7);

pub static GLOBALS_WEBEXTENSIONS: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x807b, 205>(), &generated_u16s::<0x8148, 0>(), 3);

pub static GLOBALS_QUNIT: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x8148, 205>(), &generated_u16s::<0x8215, 0>(), 19);

pub static GLOBALS_VITEST: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x8215, 205>(), &generated_u16s::<0x82e2, 0>(), 17);

pub static GLOBALS_VUE: GlobalSet =
    GlobalSet::new(&generated_bytes::<0x82e2, 205>(), &generated_u16s::<0x83af, 0>(), 7);

static ENVIRONMENTS: [(&str, &GlobalSet); 44] = [
    ("builtin", &GLOBALS_BUILTIN),
    ("es6", &GLOBALS_ES6),
    ("es2015", &GLOBALS_ES2015),
    ("es2016", &GLOBALS_ES2016),
    ("es2017", &GLOBALS_ES2017),
    ("es2018", &GLOBALS_ES2018),
    ("es2019", &GLOBALS_ES2019),
    ("es2020", &GLOBALS_ES2020),
    ("es2021", &GLOBALS_ES2021),
    ("es2022", &GLOBALS_ES2022),
    ("es2023", &GLOBALS_ES2023),
    ("es2024", &GLOBALS_ES2024),
    ("es2025", &GLOBALS_ES2025),
    ("es2026", &GLOBALS_ES2026),
    ("audioworklet", &GLOBALS_AUDIOWORKLET),
    ("browser", &GLOBALS_BROWSER),
    ("bun", &GLOBALS_BUN),
    ("node", &GLOBALS_NODE),
    ("shared-node-browser", &GLOBALS_SHARED_NODE_BROWSER),
    ("worker", &GLOBALS_WORKER),
    ("serviceworker", &GLOBALS_SERVICEWORKER),
    ("amd", &GLOBALS_AMD),
    ("applescript", &GLOBALS_APPLESCRIPT),
    ("astro", &GLOBALS_ASTRO),
    ("atomtest", &GLOBALS_ATOMTEST),
    ("commonjs", &GLOBALS_COMMONJS),
    ("embertest", &GLOBALS_EMBERTEST),
    ("greasemonkey", &GLOBALS_GREASEMONKEY),
    ("jasmine", &GLOBALS_JASMINE),
    ("jest", &GLOBALS_JEST),
    ("jquery", &GLOBALS_JQUERY),
    ("meteor", &GLOBALS_METEOR),
    ("mocha", &GLOBALS_MOCHA),
    ("mongo", &GLOBALS_MONGO),
    ("nashorn", &GLOBALS_NASHORN),
    ("protractor", &GLOBALS_PROTRACTOR),
    ("prototypejs", &GLOBALS_PROTOTYPEJS),
    ("phantomjs", &GLOBALS_PHANTOMJS),
    ("shelljs", &GLOBALS_SHELLJS),
    ("svelte", &GLOBALS_SVELTE),
    ("webextensions", &GLOBALS_WEBEXTENSIONS),
    ("qunit", &GLOBALS_QUNIT),
    ("vitest", &GLOBALS_VITEST),
    ("vue", &GLOBALS_VUE),
];

/// All available environments.
pub static GLOBALS: Globals = Globals;
