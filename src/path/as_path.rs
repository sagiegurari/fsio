//! # as_path
//!
//! AsPath trait and implementations.
//!

#[cfg(test)]
#[path = "./as_path_test.rs"]
mod as_path_test;

use std::path::{Path, PathBuf};

/// Defines as path trait.
pub trait AsPath {
    /// Converts to a path reference.
    fn as_path(&self) -> &Path;
}

impl AsPath for str {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }
}

impl AsPath for &str {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }
}

impl AsPath for String {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }
}

impl AsPath for &String {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }
}

impl AsPath for &Path {
    fn as_path(&self) -> &Path {
        self
    }
}

impl AsPath for PathBuf {
    fn as_path(&self) -> &Path {
        PathBuf::as_path(&self)
    }
}
