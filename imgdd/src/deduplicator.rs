use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::DeletePolicy;

/// The [`Deduplicator`] provides functions to scan and manage duplicated images in a directory.
pub struct Deduplicator {}

impl Deduplicator {
    /// Scans for all images in the given directories and returns a mapping of similar image paths.
    ///
    /// Under the hood, it uses the [`img_hash`](https://crates.io/crates/img_hash) crate.
    pub fn scan<D>(&self, directories: D) -> HashMap<PathBuf, Vec<PathBuf>>
    where
        D: IntoIterator<Item = Path>,
    {
        HashMap::default()
    }

    /// Automatically deletes files based on a given [deletion policy](DeletePolicy).
    ///
    /// Users will probably want to call this function after getting a value back from [`scan`](Deduplicator::scan);
    /// if no modification to the result of [`scan`](Deduplicator::scan) is needed, consider using
    /// [`scan_and_delete`](Deduplicator::scan_and_delete) for convenience.
    pub fn delete(
        &self,
        duplicate_map: HashMap<PathBuf, Vec<PathBuf>>,
        policy: DeletePolicy,
    ) -> Vec<OsString> {
        vec![]
    }

    /// Automatically scans and deletes files based on the given deletion policy, and returns the list of deleted
    /// file names.
    ///
    /// This is just a convenience function that calls [`scan`](Deduplicator::.scan) and
    /// [`delete`](Deduplicator::delete) in turn.
    pub fn scan_and_delete<D>(&self, directories: D, policy: DeletePolicy) -> Vec<OsString>
    where
        D: IntoIterator<Item = Path>,
    {
        let duplicate_map = self.scan(directories);
        self.delete(duplicate_map, policy)
    }
}

/// The [`DeduplicatorConfig`] is how one can configure the [`Deduplicator`]. The [`Deduplicator`] can
/// then be constructed using [`build`](DeduplicatorConfig::build).
pub struct DeduplicatorConfig {}

impl DeduplicatorConfig {
    /// Builds a [`Deduplicator`] out of the given [`DeduplicatorConfig`].
    pub fn build(self) -> Deduplicator {
        Deduplicator {}
    }
}
