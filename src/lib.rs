//! # JavaScript Globals
//!
//! Global identifiers from different JavaScript environments
//!
//! Rust fork of <https://www.npmjs.com/package/globals>

mod generated;
pub use generated::{GLOBALS, GLOBALS_BUILTIN, GLOBALS_ES2026};

struct GlobalNames {
    seed: u64,
    pilots: &'static [u8],
    remap: &'static [u32],
    names: &'static [u8],
    offsets: &'static [u16],
}

impl GlobalNames {
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

/// A compact map of global names to their writability.
pub struct GlobalSet {
    members: &'static [u8; generated::GLOBAL_NAME_BYTES],
    writable: &'static [u16],
}

impl GlobalSet {
    fn global_id(&self, key: &str) -> Option<u16> {
        let id = generated::GLOBAL_NAMES.get(key)?;
        self.contains_id(id).then_some(id)
    }

    fn contains_id(&self, id: u16) -> bool {
        let id = usize::from(id);
        self.members[id / 8] & (1 << (id % 8)) != 0
    }

    fn is_writable(&self, id: u16) -> bool {
        self.writable.binary_search(&id).is_ok()
    }

    /// Returns whether the map contains `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.global_id(key).is_some()
    }

    /// Returns the writability of `key`.
    pub fn get(&self, key: &str) -> Option<bool> {
        let id = self.global_id(key)?;
        Some(self.is_writable(id))
    }

    /// Returns an iterator over the entries of the globals map.
    pub fn entries(&self) -> impl Iterator<Item = (&'static str, bool)> + '_ {
        (0..generated::GLOBAL_NAME_COUNT).filter_map(|id| {
            let id = id as u16;
            self.contains_id(id)
                .then(|| (generated::GLOBAL_NAMES.name(usize::from(id)), self.is_writable(id)))
        })
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
