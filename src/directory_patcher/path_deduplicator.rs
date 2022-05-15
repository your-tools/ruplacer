use std::path::Path;

use os_str_bytes::RawOsStr;
use patricia_tree::PatriciaSet;

#[derive(Debug, Default)]
pub struct PathDeduplicator {
    set: PatriciaSet,
}

impl PathDeduplicator {
    pub fn new() -> Self {
        Self::default()
    }

    // Returns `true` if the given `path` was called for this instance before.
    pub fn insert_path(&mut self, path: &Path) -> bool {
        let Self { set } = self;
        let raw = RawOsStr::new(path.as_os_str());
        set.insert(raw.as_raw_bytes())
    }
}
