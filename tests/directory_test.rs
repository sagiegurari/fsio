extern crate fsio;

use crate::fsio::{directory, file};
use std::path::Path;

#[test]
fn create_test() {
    let result = directory::create("./target/__test/directory_test/dir1/dir2");
    assert!(result.is_ok());

    let path = Path::new("./target/__test/directory_test/dir1/dir2");
    assert!(path.exists());
}

#[test]
fn create_parent_test() {
    let result = directory::create_parent("./target/__test/directory_test/dir1/files/file.txt");
    assert!(result.is_ok());

    let path = Path::new("./target/__test/directory_test/dir1/files");
    assert!(path.exists());
}

#[test]
fn delete_test() {
    file::ensure_exists("./target/__test/directory_test/delete_directory/dir1/dir2/file.txt")
        .unwrap();
    let path = Path::new("./target/__test/directory_test/delete_directory");
    assert!(path.exists());

    let result = directory::delete("./target/__test/directory_test/delete_directory");
    assert!(result.is_ok());

    assert!(!path.exists());
}
