extern crate fsio;

use crate::fsio::file;
use std::path::Path;
use std::str;

#[test]
fn ensure_exists_test() {
    let result = file::ensure_exists("./target/__test/file_test/dir1/dir2/file.txt");
    assert!(result.is_ok());

    let path = Path::new("./target/__test/file_test/dir1/dir2/file.txt");
    assert!(path.exists());
}

#[test]
fn create_and_read_text_file_test() {
    let file_path = "./target/__test/file_test/write_text_file/file.txt";
    let result = file::write_text_file(file_path, "some content");
    assert!(result.is_ok());

    let text = file::read_text_file(file_path).unwrap();

    assert_eq!(text, "some content");
}

#[test]
fn append_and_read_text_file_test() {
    let file_path = "./target/__test/file_test/append_text_file/file.txt";
    let mut result = file::write_text_file(file_path, "some content");
    assert!(result.is_ok());
    result = file::append_text_file(file_path, "\nmore content");
    assert!(result.is_ok());

    let text = file::read_text_file(file_path).unwrap();

    assert_eq!(text, "some content\nmore content");
}

#[test]
fn append_and_read_file_test() {
    let file_path = "./target/__test/file_test/append_and_read_file_test/file.txt";
    let mut result = file::write_file(file_path, "some content".as_bytes());
    assert!(result.is_ok());
    result = file::append_file(file_path, "\nmore content".as_bytes());
    assert!(result.is_ok());

    let data = file::read_file(file_path).unwrap();

    assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
}

#[test]
fn delete_file_test() {
    let file_path = "./target/__test/file_test/delete_file/file.txt";
    let mut result = file::ensure_exists(file_path);
    assert!(result.is_ok());

    let path = Path::new(file_path);
    assert!(path.exists());

    result = file::delete(file_path);
    assert!(result.is_ok());

    assert!(!path.exists());
}
