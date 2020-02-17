//! # from_path
//!
//! FromPath trait and implementations.
//!

#[cfg(test)]
#[path = "./from_path_test.rs"]
mod from_path_test;

use std::path::{Path, PathBuf};

/// Defines as path trait.
pub trait FromPath {
    /// Converts from a path reference.
    fn from_path(path: &Path) -> Self;
}

impl FromPath for String {
    fn from_path(path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }
}

impl FromPath for PathBuf {
    fn from_path(path: &Path) -> PathBuf {
        path.to_path_buf()
    }
}
