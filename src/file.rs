//! # file
//!
//! File utility functions.
//!

#[cfg(test)]
#[path = "./file_test.rs"]
mod file_test;

use crate::directory;
use crate::error::FsIOError;
use crate::path::as_path::AsPath;
use std::fs::{read, read_to_string, remove_file, File, OpenOptions};
use std::io;
use std::io::Write;

/// Ensures the provided path leads to an existing file.
/// If the file does not exist, this function will create an emtpy file.
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
///
/// fn main() {
///     let result = file::ensure_exists("./target/__test/file_test/dir1/dir2/file.txt");
///     assert!(result.is_ok());
///
///     let path = Path::new("./target/__test/file_test/dir1/dir2/file.txt");
///     assert!(path.exists());
/// }
/// ```
pub fn ensure_exists<T: AsPath + ?Sized>(path: &T) -> Result<(), FsIOError> {
    let file_path = path.as_path();

    if file_path.exists() {
        if file_path.is_file() {
            Ok(())
        } else {
            Err(FsIOError::PathAlreadyExists(
                    format!("Unable to create file: {:?}", &file_path).to_string(),
                ),
            )
        }
    } else {
        directory::create_parent(path)?;

        match File::create(&file_path) {
            Ok(_) => Ok(()),
            Err(error) => Err(FsIOError::IOError(
                    format!("Unable to create file: {:?}", &file_path).to_string(),
                    Some(error),
                ),
            ),
        }
    }
}

/// Creates and writes the text to the requested file path.
/// If a file exists at that path, it will be overwritten.
///
/// # Arguments
///
/// * `path` - The file path
/// * `text` - The file text content
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/write_text_file/file.txt";
///     let result = file::write_text_file(file_path, "some content");
///     assert!(result.is_ok());
///
///     let text = file::read_text_file(file_path).unwrap();
///
///     assert_eq!(text, "some content");
/// }
/// ```
pub fn write_text_file<T: AsPath + ?Sized>(path: &T, text: &str) -> Result<(), FsIOError> {
    write_file(path, text.as_bytes())
}

/// Appends (or creates) and writes the text to the requested file path.
/// If a file exists at that path, the content will be appended.
///
/// # Arguments
///
/// * `path` - The file path
/// * `text` - The file text content
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/append_text_file/file.txt";
///     let mut result = file::write_text_file(file_path, "some content");
///     assert!(result.is_ok());
///     result = file::append_text_file(file_path, "\nmore content");
///     assert!(result.is_ok());
///
///     let text = file::read_text_file(file_path).unwrap();
///
///     assert_eq!(text, "some content\nmore content");
/// }
/// ```
pub fn append_text_file<T: AsPath + ?Sized>(path: &T, text: &str) -> Result<(), FsIOError> {
    append_file(path, text.as_bytes())
}

/// Creates and writes the raw data to the requested file path.
/// If a file exists at that path, it will be overwritten.
///
/// # Arguments
///
/// * `path` - The file path
/// * `data` - The file raw content
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/write_file/file.txt";
///     let mut result = file::write_file(file_path, "some content".as_bytes());
///     assert!(result.is_ok());
///     result = file::append_file(file_path, "\nmore content".as_bytes());
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn write_file<T: AsPath + ?Sized>(path: &T, data: &[u8]) -> Result<(), FsIOError> {
    modify_file(path, &move |file: &mut File| file.write_all(data), false)
}

/// Appends (or creates) and writes the raw data to the requested file path.
/// If a file exists at that path, the content will be appended.
///
/// # Arguments
///
/// * `path` - The file path
/// * `data` - The file raw content
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/append_file/file.txt";
///     let mut result = file::write_file(file_path, "some content".as_bytes());
///     assert!(result.is_ok());
///     result = file::append_file(file_path, "\nmore content".as_bytes());
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn append_file<T: AsPath + ?Sized>(path: &T, data: &[u8]) -> Result<(), FsIOError> {
    modify_file(path, &move |file: &mut File| file.write_all(data), true)
}

/// Overwrites or appends the requested file and triggers the provided write_content function to
/// enable custom writing.
///
/// # Arguments
///
/// * `path` - The file path
/// * `write_content` - The custom writing function
/// * `append` - True to append false to overwrite
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::fs::File;
/// use std::io::Write;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/modify_file/file.txt";
///     let mut result = file::modify_file(
///         file_path,
///         &move |file: &mut File| file.write_all("some content".as_bytes()),
///         false,
///     );
///     assert!(result.is_ok());
///     result = file::modify_file(
///         file_path,
///         &move |file: &mut File| file.write_all("\nmore content".as_bytes()),
///         true,
///     );
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn modify_file<T: AsPath + ?Sized>(
    path: &T,
    write_content: &Fn(&mut File) -> io::Result<()>,
    append: bool,
) -> Result<(), FsIOError> {
    directory::create_parent(path)?;

    let file_path = path.as_path();

    // create or open
    let result = if append && file_path.exists() {
        OpenOptions::new().append(true).open(file_path)
    } else {
        File::create(&file_path)
    };

    match result {
        Ok(mut fd) => match write_content(&mut fd) {
            Ok(_) => match fd.sync_all() {
                Ok(_) => Ok(()),
                Err(error) => Err(FsIOError::IOError(
                        format!("Error finish up writing to file: {:?}", &file_path).to_string(),
                        Some(error),
                    ),
                ),
            },
            Err(error) => Err(FsIOError::IOError(
                    format!("Error while writing to file: {:?}", &file_path).to_string(),
                    Some(error),
                ),
            ),
        },
        Err(error) => Err(FsIOError::IOError(
                format!("Unable to create/open file: {:?} for writing.", &file_path).to_string(),
                Some(error),
            ),
        ),
    }
}

/// Reads the requested text file and returns its content.
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/write_text_file/file.txt";
///     let result = file::write_text_file(file_path, "some content");
///     assert!(result.is_ok());
///
///     let text = file::read_text_file(file_path).unwrap();
///
///     assert_eq!(text, "some content");
/// }
/// ```
pub fn read_text_file<T: AsPath + ?Sized>(path: &T) -> Result<String, FsIOError> {
    let file_path = path.as_path();

    match read_to_string(&file_path) {
        Ok(content) => Ok(content),
        Err(error) => Err(FsIOError::IOError(
                format!("Unable to read file: {:?}", &file_path).to_string(),
                Some(error),
            ),
        ),
    }
}

/// Reads the requested file and returns its content.
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/read_file/file.txt";
///     let mut result = file::write_file(file_path, "some content".as_bytes());
///     assert!(result.is_ok());
///     result = file::append_file(file_path, "\nmore content".as_bytes());
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn read_file<T: AsPath + ?Sized>(path: &T) -> Result<Vec<u8>, FsIOError> {
    let file_path = path.as_path();

    match read(&file_path) {
        Ok(content) => Ok(content),
        Err(error) => Err(FsIOError::IOError(
                format!("Unable to read file: {:?}", &file_path).to_string(),
                Some(error),
            ),
        ),
    }
}

/// Deletes the requested file.
/// If the file does not exist, this function will return valid response.
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/delete_file/file.txt";
///     let mut result = file::ensure_exists(file_path);
///     assert!(result.is_ok());
///
///     let path = Path::new(file_path);
///     assert!(path.exists());
///
///     result = file::delete(file_path);
///     assert!(result.is_ok());
///
///     assert!(!path.exists());
/// }
/// ```
pub fn delete<T: AsPath + ?Sized>(path: &T) -> Result<(), FsIOError> {
    let file_path = path.as_path();

    if file_path.exists() {
        if file_path.is_file() {
            match remove_file(file_path) {
                Ok(_) => Ok(()),
                Err(error) => Err(FsIOError::IOError(
                        format!("Unable to delete file: {:?}", &file_path).to_string(),
                        Some(error),
                    ),
                ),
            }
        } else {
            Err(FsIOError::NotFile(
                    format!("Path: {:?} is not a file.", &file_path).to_string(),
                ),
            )
        }
    } else {
        Ok(())
    }
}

/// Deletes the requested file.
/// If the file does not exist, this function will return true.
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Example
///
/// ```
/// use crate::fsio::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/delete_file/file.txt";
///     let result = file::ensure_exists(file_path);
///     assert!(result.is_ok());
///
///     let path = Path::new(file_path);
///     assert!(path.exists());
///
///     let deleted = file::delete_ignore_error(file_path);
///     assert!(deleted);
///
///     assert!(!path.exists());
/// }
/// ```
pub fn delete_ignore_error<T: AsPath + ?Sized>(path: &T) -> bool {
    match delete(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
