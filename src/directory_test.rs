use super::*;

use crate::file::ensure_exists;
use std::fs::File;
use std::path::Path;

#[test]
fn create_with_parent_directories() {
    let path_str = "./target/__test/ut/directory_test/create_with_parent_directories/dir1/dir2/";
    let path = Path::new(path_str);
    delete(path_str).unwrap();
    assert!(!path.exists());

    let result =
        create("./target/__test/ut/directory_test/create_with_parent_directories/dir1/dir2");
    assert!(result.is_ok());

    assert!(path.exists());
    delete(path_str).unwrap();
}

#[test]
fn create_already_exists() {
    let path_str = "./target/__test/ut/directory_test/create_already_exists/dir1/dir2";
    let path = Path::new(path_str);
    delete(path_str).unwrap();
    assert!(!path.exists());

    let mut result = create("./target/__test/ut/directory_test/create_already_exists/dir1/dir2");
    assert!(result.is_ok());

    assert!(path.exists());

    result = create("./target/__test/ut/directory_test/create_already_exists/dir1/dir2");
    assert!(result.is_ok());
    delete(path_str).unwrap();
}

#[test]
fn create_on_file() {
    let path_str = "./target/__test/ut/directory_test/create_on_file";
    let path = Path::new(path_str);
    delete(path_str).unwrap();
    assert!(!path.exists());

    let mut result = create("./target/__test/ut/directory_test/create_on_file");
    assert!(result.is_ok());

    let file_result = File::create("./target/__test/ut/directory_test/create_on_file/file.txt");
    assert!(file_result.is_ok());

    result = create("./target/__test/ut/directory_test/create_on_file/file.txt");
    assert!(result.is_err());
    delete(path_str).unwrap();
}

#[test]
fn create_parent_valid() {
    let path_str = "./target/__test/ut/directory_test/create_parent_valid/";
    let mut path = Path::new(path_str);
    delete(path_str).unwrap();
    assert!(!path.exists());

    let result = create_parent("./target/__test/ut/directory_test/create_parent_valid/file.txt");
    assert!(result.is_ok());

    assert!(path.exists());

    path = Path::new("./target/__test/ut/directory_test/create_parent_valid/file.txt");
    assert!(!path.exists());
    delete(path_str).unwrap();
}

#[test]
fn create_parent_on_file() {
    let path_str = "./target/__test/ut/directory_test/create_parent_on_file/";
    let path = Path::new(path_str);
    delete(path_str).unwrap();
    assert!(!path.exists());

    let mut result = create("./target/__test/ut/directory_test/create_parent_on_file");
    assert!(result.is_ok());

    let file_result = File::create("./target/__test/ut/directory_test/create_parent_on_file/files");
    assert!(file_result.is_ok());

    result = create("./target/__test/ut/directory_test/create_parent_on_file/files/file.txt");
    assert!(result.is_err());
    delete(path_str).unwrap();
}

#[test]
fn create_parent_no_parent() {
    let result = create("/");
    assert!(result.is_ok());
}

#[test]
fn delete_exists() {
    ensure_exists(
        "./target/__test/ut/directory_test/delete_directory/delete_exists/dir1/dir2/file.txt",
    )
    .unwrap();
    let path = Path::new("./target/__test/ut/directory_test/delete_directory/delete_exists");
    assert!(path.exists());

    let result = delete("./target/__test/ut/directory_test/delete_directory/delete_exists");
    assert!(result.is_ok());

    assert!(!path.exists());
}

#[test]
fn delete_not_exists() {
    let path = Path::new("./target/__test/ut/directory_test/delete_directory/delete_not_exists");
    assert!(!path.exists());

    let result = delete("./target/__test/ut/directory_test/delete_directory/delete_not_exists");
    assert!(result.is_ok());

    assert!(!path.exists());
}

#[test]
fn delete_file() {
    let file_path =
        "./target/__test/ut/directory_test/delete_directory/delete_file/dir1/dir2/file.txt";
    ensure_exists(file_path).unwrap();
    let path = Path::new(file_path);
    assert!(path.exists());

    let result = delete(file_path);
    assert!(result.is_err());

    assert!(path.exists());
}
