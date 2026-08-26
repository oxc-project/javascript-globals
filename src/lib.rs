//! # JavaScript Globals
//!
//! Global identifiers from different JavaScript environments
//!
//! Rust fork of <https://www.npmjs.com/package/globals>
//!
//! ## Representation
//!
//! Every unique global name has one shared `u16` ID. A [`GlobalSet`] stores one membership bit
//! per ID and a sorted list of the writable IDs, so environments do not repeat names or hash-table
//! metadata. Identical environments share the same static set.
//!
//! The generated `data.bin` contains the PHF tables, concatenated UTF-8 names, name offsets,
//! membership bitsets, and writable-ID lists. The `generated` module converts those byte ranges
//! into typed static arrays during const evaluation; no data is parsed or allocated at runtime.
//!
//! A name lookup uses the shared PHF to find a candidate ID, verifies the candidate string, checks
//! the environment's membership bit, then binary-searches its writable IDs. Iteration scans set
//! bits and uses a const-generated reference table into the shared name blob.
//!
//! Environment lookup uses a generated string match. The separate environment array is referenced
//! only by iteration, allowing linkers to discard it when callers use only [`Globals::get`].

pub use generated::{GLOBALS, GLOBALS_BUILTIN, GLOBALS_ES2026};

/// A compact map of global names to their writability.
pub struct GlobalSet {
    /// One bit for every ID in the shared global-name index.
    members: &'static [u8; generated::GLOBAL_NAME_BYTES],
    /// Sorted IDs for the writable members of this environment; other members are read-only.
    writable: &'static [u16],
}

impl GlobalSet {
    /// Returns whether the map contains `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.global_id(key).is_some()
    }

    /// Returns the writability of `key`.
    pub fn get(&self, key: &str) -> Option<&'static bool> {
        let id = self.global_id(key)?;
        Some(self.writable_value(id))
    }

    /// Returns an iterator over names.
    pub fn keys(&self) -> impl Iterator<Item = &'static &'static str> + '_ {
        self.into_iter().map(|entry| entry.0)
    }
}

impl<'a> IntoIterator for &'a GlobalSet {
    type Item = (&'static &'static str, &'static bool);
    type IntoIter = GlobalEntries<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GlobalEntries { set: self, ids: 0..generated::GLOBAL_NAME_COUNT }
    }
}

/// Iterator over a [`GlobalSet`]'s entries.
pub struct GlobalEntries<'a> {
    set: &'a GlobalSet,
    ids: core::ops::Range<usize>,
}

impl Iterator for GlobalEntries<'_> {
    type Item = (&'static &'static str, &'static bool);

    fn next(&mut self) -> Option<Self::Item> {
        while self.ids.start < self.ids.end {
            let byte_index = self.ids.start / 8;
            // Ignore IDs already visited in this byte, then jump directly to its next set bit.
            let byte = self.set.members[byte_index] & (u8::MAX << (self.ids.start % 8));
            if byte != 0 {
                let id = byte_index * 8 + byte.trailing_zeros() as usize;
                if id < self.ids.end {
                    self.ids.start = id + 1;
                    return Some(self.set.entry(id as u16));
                }
            }
            self.ids.start = ((byte_index + 1) * 8).min(self.ids.end);
        }
        None
    }
}

/// A map of environment names to their global variable maps.
pub struct Globals;

impl Globals {
    /// Returns an iterator over the entries of the globals map.
    pub fn entries(&self) -> impl Iterator<Item = (&'static str, &'static GlobalSet)> + '_ {
        generated::ENVIRONMENTS.iter().copied()
    }

    /// Returns the globals map for the given environment name.
    pub fn get(&self, key: &str) -> Option<&'static GlobalSet> {
        generated::get_environment(key)
    }
}

mod generated;

/// Shared perfect-hash index and compact name storage.
struct GlobalNames {
    seed: u64,
    pilots: &'static [u8],
    remap: &'static [u32],
    /// Every name concatenated in PHF slot order.
    names: &'static [u8],
    /// One byte offset per name plus a final sentinel.
    offsets: &'static [u16],
}

impl GlobalNames {
    /// Returns the ID only when the PHF candidate contains the exact input string.
    fn get(&self, name: &str) -> Option<u16> {
        let hash = phf_shared::ptrhash::hash(name, &self.seed);
        let index = phf_shared::ptrhash::get_index(
            self.seed,
            hash,
            self.pilots,
            self.remap,
            generated::GLOBAL_NAME_COUNT,
        ) as usize;
        (self.name(index) == name).then_some(index as u16)
    }

    fn name(&self, id: usize) -> &'static str {
        // SAFETY: `offsets` is generated with one valid boundary per name plus a final sentinel,
        // PHF always returns an ID below `GLOBAL_NAME_COUNT`, and the byte blob is built from
        // valid UTF-8 strings.
        unsafe {
            let start = usize::from(*self.offsets.get_unchecked(id));
            let end = usize::from(*self.offsets.get_unchecked(id + 1));
            core::str::from_utf8_unchecked(self.names.get_unchecked(start..end))
        }
    }
}

impl GlobalSet {
    /// Resolves `key` in the shared name index, then checks this environment's membership bit.
    fn global_id(&self, key: &str) -> Option<u16> {
        let id = generated::GLOBAL_NAMES.get(key)?;
        self.contains_id(id).then_some(id)
    }

    fn contains_id(&self, id: u16) -> bool {
        let id = usize::from(id);
        self.members[id / 8] & (1 << (id % 8)) != 0
    }

    fn writable_value(&self, id: u16) -> &'static bool {
        if self.writable.binary_search(&id).is_ok() { &true } else { &false }
    }

    fn entry(&self, id: u16) -> (&'static &'static str, &'static bool) {
        (&generated::GLOBAL_NAME_REFS[usize::from(id)], self.writable_value(id))
    }
}
