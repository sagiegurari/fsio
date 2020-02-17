//! # directory
//!
//! Directory utility functions.
//!

#[cfg(test)]
#[path = "./directory_test.rs"]
mod directory_test;

use crate::error::{ErrorInfo, FsIOError};
use crate::path::as_path::AsPath;
use crate::path::get_parent_directory;
use std::fs::{create_dir_all, remove_dir_all};

/// Creates the directory (and if needed the parent directories) for the provided path.
///
/// # Arguments
///
/// * `path` - The directory path
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use crate::fsio::directory;
/// use std::path::Path;
///
/// fn main() {
///     let result = directory::create("./target/__test/directory_test/dir1/dir2");
///     assert!(result.is_ok());
///
///     let path = Path::new("./target/__test/directory_test/dir1/dir2");
///     assert!(path.exists());
/// }
/// ```
pub fn create<T: AsPath + ?Sized>(path: &T) -> Result<(), FsIOError> {
    let directory_path = path.as_path();

    if directory_path.is_dir() && directory_path.exists() {
        return Ok(());
    }

    match create_dir_all(&directory_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(FsIOError {
            info: ErrorInfo::IOError(
                format!("Unable to create directory: {:?}.", &directory_path).to_string(),
                Some(error),
            ),
        }),
    }
}

/// Creates the parent directory (and if needed the parent directories) for the provided path.
/// In case no parent directory path component exists, this function will return ok result.
///
/// # Arguments
///
/// * `path` - The child path
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use crate::fsio::directory;
/// use std::path::Path;
///
/// fn main() {
///     let result = directory::create_parent("./target/__test/directory_test/dir1/files/file.txt");
///     assert!(result.is_ok());
///
///     let path = Path::new("./target/__test/directory_test/dir1/files");
///     assert!(path.exists());
/// }
/// ```
pub fn create_parent<T: AsPath + ?Sized>(path: &T) -> Result<(), FsIOError> {
    match get_parent_directory(path) {
        Some(directory) => create(&directory),
        None => Ok(()),
    }
}

/// Deletes the directory and any child file/directory.
///
/// # Arguments
///
/// * `path` - The directory path
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use crate::fsio::{directory, file};
/// use std::path::Path;
///
/// fn main() {
///     file::ensure_exists("./target/__test/directory_test/delete_directory/dir1/dir2/file.txt").unwrap();
///     let path = Path::new("./target/__test/directory_test/delete_directory");
///     assert!(path.exists());
///
///     let result = directory::delete("./target/__test/directory_test/delete_directory");
///     assert!(result.is_ok());
///
///     assert!(!path.exists());
/// }
/// ```
pub fn delete<T: AsPath + ?Sized>(path: &T) -> Result<(), FsIOError> {
    let directory_path = path.as_path();

    if directory_path.exists() {
        if directory_path.is_dir() {
            match remove_dir_all(directory_path) {
                Ok(_) => Ok(()),
                Err(error) => Err(FsIOError {
                    info: ErrorInfo::IOError(
                        format!("Unable to delete directory: {:?}", &directory_path).to_string(),
                        Some(error),
                    ),
                }),
            }
        } else {
            Err(FsIOError {
                info: ErrorInfo::NotFile(
                    format!("Path: {:?} is not a directory.", &directory_path).to_string(),
                ),
            })
        }
    } else {
        Ok(())
    }
}
