use bk_tree::BKTree;
use img_hash::image;
use itertools::Itertools;
use std::{collections::HashMap, ffi::OsString, path::PathBuf};
use walkdir::WalkDir;

use crate::hamming_distance_metric::HammingDistance;

/// The [`Scanner`] provides functions to scan for duplicated images in directories.
///
/// Under the hood, it uses the [`img_hash`](https://crates.io/crates/img_hash) crate.
pub struct Scanner {
    recursive: bool,
}

impl Scanner {
    /// Given an iterator of paths, returns a graph of images that are detected to be similar to each other.
    pub fn scan<P>(&self, paths: P) -> HashMap<OsString, Vec<OsString>>
    where
        P: IntoIterator<Item = PathBuf>,
    {
        let hasher = img_hash::HasherConfig::with_bytes_type::<[u8; 8]>().to_hasher();
        let mut tree: BKTree<(OsString, [u8; 8]), HammingDistance> = BKTree::new(HammingDistance);
        let mut file_hashes: HashMap<OsString, [u8; 8]> = HashMap::new();

        paths
            .into_iter()
            .map(|path| {
                if self.recursive {
                    WalkDir::new(path)
                } else {
                    WalkDir::new(path).max_depth(0)
                }
            })
            .for_each(|walk_dir| {
                walk_dir.into_iter().for_each(|dir_entry| {
                    if let Ok(dir_entry) = dir_entry {
                        let path = dir_entry.path();
                        if let Ok(img) = image::open(path) {
                            let hash = hasher.hash_image(&img);
                            let mut bytes: [u8; 8] = [0; 8];
                            let name = path.as_os_str().to_os_string();
                            bytes.copy_from_slice(hash.as_bytes());

                            // TODO: Is there some way to avoid this clone...? Maybe we create the hashmap first and add all at once?
                            file_hashes.insert(name.clone(), bytes);

                            tree.add((name, bytes));
                        }
                    }
                })
            });

        file_hashes
            .into_iter()
            .filter_map(|(file, hash)| {
                let similar = tree
                    .find(&(file.clone(), hash), 10)
                    .sorted_by_key(|res| res.0)
                    .map(|res| (res.1).0.clone())
                    .collect::<Vec<_>>();

                if similar.is_empty() {
                    Some((file, similar))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// The [`ScannerConfig`] is how one can configure the [`Scanner`]. The [`Scanner`] can
/// then be constructed using [`build`](ScannerConfig::build).
pub struct ScannerConfig {
    recursive: bool,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            recursive: Default::default(),
        }
    }
}

impl ScannerConfig {
    /// Builds a [`Scanner`] out of the given [`ScannerConfig`].
    pub fn build(self) -> Scanner {
        Scanner {
            recursive: self.recursive,
        }
    }
}
