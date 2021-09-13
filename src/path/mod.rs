//! # path
//!
//! Path utility functions and traits.
//!

pub mod as_path;
pub mod from_path;

#[cfg(feature = "temp-path")]
mod temp_path;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::error::FsIOError;
use crate::types::FsIOResult;
use as_path::AsPath;
use from_path::FromPath;
use std::fs;
use std::time::SystemTime;

/// Returns a canonicalized string from the provided path value.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let path_obj = Path::new("./src/path/mod.rs");
///
///     let path1 = path::canonicalize_as_string(&path_obj);
///     let path2 = path::canonicalize_as_string(&"./src/path/mod.rs".to_string());
///
///     assert_eq!(path1.unwrap(), path2.unwrap());
/// }
/// ```
pub fn canonicalize_as_string<T: AsPath + ?Sized>(path: &T) -> FsIOResult<String> {
    let path_obj = path.as_path();

    match path_obj.canonicalize() {
        Ok(path_buf) => {
            let path_string = FromPath::from_path(&path_buf);

            #[cfg(not(windows))]
            {
                Ok(path_string)
            }
            #[cfg(windows)]
            {
                let win_path_string = match dunce::canonicalize(path_string) {
                    Ok(value) => FromPath::from_path(&value),
                    Err(_) => path_string.to_string(),
                };

                Ok(win_path_string);
            }
        }
        Err(error) => Err(FsIOError::IOError(
            "Unable to canonicalize path.".to_string(),
            Some(error),
        )),
    }
}

/// Returns a canonicalized string from the provided path value.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let path_obj = Path::new("./src/path/mod.rs");
///
///     let path1 = path::canonicalize_as_string(&path_obj);
///     let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");
///
///     assert_eq!(path1.unwrap(), path2);
/// }
/// ```
pub fn canonicalize_or<T: AsPath + ?Sized>(path: &T, or_value: &str) -> String {
    match canonicalize_as_string(path) {
        Ok(value) => value,
        Err(_) => or_value.to_string(),
    }
}

/// Returns the last path component (file name or last directory name).
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let basename = path::get_basename("./src/path/mod.rs");
///
///     assert_eq!(basename.unwrap(), "mod.rs");
/// }
/// ```
pub fn get_basename<T: AsPath + ?Sized>(path: &T) -> Option<String> {
    let path_obj = path.as_path();

    match path_obj.file_name() {
        Some(name) => Some(name.to_string_lossy().into_owned()),
        None => None,
    }
}

/// Returns the parent path.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let dirname = path::get_parent_directory("./src/path/mod.rs");
///
///     assert_eq!(dirname.unwrap(), "./src/path");
/// }
/// ```
pub fn get_parent_directory<T: AsPath + ?Sized>(path: &T) -> Option<String> {
    let path_obj = path.as_path();

    let directory = path_obj.parent();
    match directory {
        Some(directory_path) => {
            let directory_path_string: String = FromPath::from_path(directory_path);

            if directory_path_string.is_empty() {
                None
            } else {
                Some(directory_path_string)
            }
        }
        None => None,
    }
}

/// Returns the last modified time of the provided path in millies since
/// unix epoch time.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// use fsio::path;
///
/// fn main() {
///     let time = path::get_last_modified_time("./src/path/mod.rs").unwrap();
///
///     assert!(time > 0);
/// }
/// ```
pub fn get_last_modified_time(path: &str) -> FsIOResult<u128> {
    match fs::metadata(path) {
        Ok(metadata) => match metadata.modified() {
            Ok(time) => match time.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => Ok(duration.as_millis()),
                Err(error) => Err(FsIOError::SystemTimeError(
                    "Unable to get last modified duration for path.".to_string(),
                    Some(error),
                )),
            },
            Err(error) => Err(FsIOError::IOError(
                "Unable to extract modified time for path.".to_string(),
                Some(error),
            )),
        },
        Err(error) => Err(FsIOError::IOError(
            "Unable to extract metadata for path.".to_string(),
            Some(error),
        )),
    }
}

/// Returns a temporary file path.
/// The file does not exist after this function exists and can be used to create
/// a file in the OS temporary directory.
///
/// # Arguments
///
/// * `extension` - The file extension
///
/// # Feature
///
/// This function requires that the **temp-path** feature will be used.
///
/// # Example
///
/// ```
/// use fsio::path;
///
/// fn main() {
///     let temp_file = path::get_temporary_file_path("txt");
///
///     assert!(temp_file.ends_with(".txt"));
/// }
/// ```
#[cfg(feature = "temp-path")]
pub fn get_temporary_file_path(extension: &str) -> String {
    temp_path::get(extension)
}
