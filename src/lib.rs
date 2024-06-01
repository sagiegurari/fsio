#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # fsio
//!
//! File System and Path utility functions.
//!
//! # Usage
//!
//! There are multiple main modules for fsio:
//!
//! * fsio::path - Holds path related functions and traits. They do not directly modify the file system.
//! * fsio::file - File utility functions such as read_file, write_file, ...
//! * fsio::directory - Directory specific utility functions.
//!
//! ### Examples
//!
//! ```rust
//! use fsio::{directory, file, path};
//! use std::fs::File;
//! use std::io::Write;
//! use std::path::Path;
//! use std::str;
//!
//! fn main() {
//!     // file operations
//!     let mut result = file::ensure_exists("./target/__test/doc/example/file_test/dir1/dir2/file.txt");
//!     assert!(result.is_ok());
//!
//!     // create/append and read text files
//!     let mut file_path = "./target/__test/example/doc/file_test/append_text_file/file.txt";
//!     result = file::write_text_file(file_path, "some content");
//!     assert!(result.is_ok());
//!     result = file::append_text_file(file_path, "\nmore content");
//!     assert!(result.is_ok());
//!     let mut text = file::read_text_file(file_path).unwrap();
//!     assert_eq!(text, "some content\nmore content");
//!
//!     // create/append and read binary files
//!     file_path = "./target/__test/example/doc/file_test/append_and_read_file_test/file.txt";
//!     result = file::write_file(file_path, "some content".as_bytes());
//!     assert!(result.is_ok());
//!     result = file::append_file(file_path, "\nmore content".as_bytes());
//!     assert!(result.is_ok());
//!     let data = file::read_file(file_path).unwrap();
//!     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
//!
//!     // custom writing
//!     file_path = "./target/__test/example/doc/file_test/modify_file/file.txt";
//!     result = file::modify_file(
//!         file_path,
//!         &move |file: &mut File| file.write_all("some content".as_bytes()),
//!         false,
//!     );
//!     assert!(result.is_ok());
//!     text = file::read_text_file(file_path).unwrap();
//!     assert_eq!(text, "some content");
//!
//!     // delete file
//!     result = file::delete(file_path);
//!     assert!(result.is_ok());
//!
//!     // directory operations
//!     result = directory::create("./target/__test/example/doc/directory_test/dir1/dir2");
//!     assert!(result.is_ok());
//!
//!     result = directory::create_parent("./target/__test/example/doc/directory_test/dir1/files/file.txt");
//!     assert!(result.is_ok());
//!
//!     // delete directory
//!     result = directory::delete("./target/__test/example/doc/directory_test");
//!     assert!(result.is_ok());
//!
//!     // basename and parent directory examples
//!     let basename = path::get_basename("./src/path/mod.rs");
//!     assert_eq!(basename.unwrap(), "mod.rs");
//!
//!     let dirname = path::get_parent_directory("./src/path/mod.rs");
//!     assert_eq!(dirname.unwrap(), "./src/path");
//!
//!     // canonicalize examples
//!     let path_obj = Path::new("./src/path/mod.rs");
//!
//!     let path1 = path::canonicalize_as_string(&path_obj);
//!     let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");
//!
//!     assert_eq!(path1.unwrap(), path2);
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! fsio = "*"
//! ```
//!
//! If you need access to temporary file paths, enable the **temp-path** feature as follows:
//!
//! ```ini
//! [dependencies]
//! fsio = { version = "*", features = ["temp-path"] }
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/fsio/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/fsio/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
use doc_comment as _;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod directory;
pub mod error;
pub mod file;
pub mod path;
pub mod types;
