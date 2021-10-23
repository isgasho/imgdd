//! `imgdd` is a crate that uses perceptual hashing (via the [img_hash](https://github.com/abonander/img_hash) crate) to
//! detect duplicate images in given directories, and providing tools to help clean these images up.

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

mod deduplicator;
mod delete_policy;
mod error;

pub use deduplicator::{Deduplicator, DeduplicatorConfig};
pub use delete_policy::DeletePolicy;
pub use error::*;
