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

mod generated;
pub use generated::*;

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
            generated::GLOBAL_NAME_COUNT,
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

/// A compact map of global names to their writability.
#[derive(PartialEq, Eq)]
pub struct GlobalSet {
    members: &'static [u8; generated::GLOBAL_NAME_BYTES],
    writable: &'static [u16],
    len: u16,
}

impl GlobalSet {
    const fn new(
        members: &'static [u8; generated::GLOBAL_NAME_BYTES],
        writable: &'static [u16],
        len: u16,
    ) -> Self {
        Self { members, writable, len }
    }

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
        Some(self.writable_value(id))
    }

    /// Returns the canonical stored key.
    pub fn get_key(&self, key: &str) -> Option<&'static &'static str> {
        let id = self.global_id(key)?;
        Some(&generated::GLOBAL_NAME_REFS[usize::from(id)])
    }

    /// Returns the canonical stored key and its writability.
    pub fn get_entry(&self, key: &str) -> Option<(&'static &'static str, &'static bool)> {
        let id = self.global_id(key)?;
        Some(self.entry(id))
    }

    /// Returns an iterator over names and their writability.
    pub fn entries(&self) -> GlobalEntries<'_> {
        GlobalEntries { set: self, ids: 0..generated::GLOBAL_NAME_COUNT, remaining: self.len() }
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
                    return Some(self.set.entry(id as u16));
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
                    return Some(self.set.entry(id as u16));
                }
            }
            self.ids.end = (byte_index * 8).max(self.ids.start);
        }
        None
    }
}

impl ExactSizeIterator for GlobalEntries<'_> {}
impl FusedIterator for GlobalEntries<'_> {}

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
        generated::ENVIRONMENTS.iter().copied()
    }

    /// Returns the globals map for the given environment name.
    pub fn get(&self, key: &str) -> Option<&'static GlobalSet> {
        generated::get_environment(key)
    }

    /// Returns an iterator over the values of the globals map.
    pub fn values(&self) -> impl Iterator<Item = &'static GlobalSet> + '_ {
        generated::ENVIRONMENTS.iter().map(|entry| entry.1)
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
