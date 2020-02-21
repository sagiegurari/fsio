use super::*;

use std::io::ErrorKind;
use std::path::Path;
use std::str;

#[test]
fn ensure_exists_not_exists() {
    let path =
        Path::new("./target/__test/ut/file_test/ensure_exists_not_exists/dir1/dir2/file.txt");
    assert!(!path.exists());

    let result =
        ensure_exists("./target/__test/ut/file_test/ensure_exists_not_exists/dir1/dir2/file.txt");
    assert!(result.is_ok());

    assert!(path.exists());
}

#[test]
fn ensure_exists_exists() {
    let path = Path::new("./target/__test/ut/file_test/ensure_exists_exists/dir1/dir2/file.txt");
    assert!(!path.exists());

    let mut result =
        ensure_exists("./target/__test/ut/file_test/ensure_exists_exists/dir1/dir2/file.txt");
    assert!(result.is_ok());

    assert!(path.exists());

    result = ensure_exists("./target/__test/ut/file_test/ensure_exists_exists/dir1/dir2/file.txt");
    assert!(result.is_ok());
}

#[test]
fn ensure_exists_on_directory() {
    let path =
        Path::new("./target/__test/ut/file_test/ensure_exists_on_directory/dir1/dir2/file.txt");
    assert!(!path.exists());

    let mut result = directory::create(
        "./target/__test/ut/file_test/ensure_exists_on_directory/dir1/dir2/file.txt",
    );
    assert!(result.is_ok());

    assert!(path.exists());

    result =
        ensure_exists("./target/__test/ut/file_test/ensure_exists_on_directory/dir1/dir2/file.txt");
    assert!(result.is_err());
}

#[test]
fn write_text_file_not_exists() {
    let file_path =
        "./target/__test/ut/file_test/write_text_file/write_text_file_not_exists/file.txt";
    let result = write_text_file(file_path, "some content");
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content");
}

#[test]
fn write_text_file_exists() {
    let file_path = "./target/__test/ut/file_test/write_text_file/write_text_file_exists/file.txt";
    let mut result = write_text_file(file_path, "some content 1");
    assert!(result.is_ok());
    result = write_text_file(file_path, "some content 2");
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content 2");
}

#[test]
fn write_text_file_on_directory() {
    let file_path = "./target/__test/ut/file_test/write_text_file/write_text_file_on_directory";
    let mut result = directory::create(file_path);
    assert!(result.is_ok());
    result = write_text_file(file_path, "some content");
    assert!(result.is_err());
}

#[test]
fn append_text_file_not_exists() {
    let file_path =
        "./target/__test/ut/file_test/append_text_file/append_text_file_not_exists/file.txt";
    let result = append_text_file(file_path, "some content");
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content");
}

#[test]
fn append_text_file_exists() {
    let file_path =
        "./target/__test/ut/file_test/append_text_file/append_text_file_exists/file.txt";
    let mut result = append_text_file(file_path, "some content 1");
    assert!(result.is_ok());
    result = append_text_file(file_path, "some content 2");
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content 1some content 2");
}

#[test]
fn append_text_file_on_directory() {
    let file_path = "./target/__test/ut/file_test/append_text_file/append_text_file_on_directory";
    let mut result = directory::create(file_path);
    assert!(result.is_ok());
    result = append_text_file(file_path, "some content");
    assert!(result.is_err());
}

#[test]
fn write_file_not_exists() {
    let file_path = "./target/__test/ut/file_test/write_file/write_file_not_exists/file.txt";
    let result = write_file(file_path, "some content".as_bytes());
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content");
}

#[test]
fn write_file_exists() {
    let file_path = "./target/__test/ut/file_test/write_file/write_file_exists/file.txt";
    let mut result = write_file(file_path, "some content 1".as_bytes());
    assert!(result.is_ok());
    result = write_file(file_path, "some content 2".as_bytes());
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content 2");
}

#[test]
fn write_file_on_directory() {
    let file_path = "./target/__test/ut/file_test/write_file/write_file_on_directory";
    let mut result = directory::create(file_path);
    assert!(result.is_ok());
    result = write_file(file_path, "some content".as_bytes());
    assert!(result.is_err());
}

#[test]
fn append_file_not_exists() {
    let file_path = "./target/__test/ut/file_test/append_file/append_file_not_exists/file.txt";
    let result = append_file(file_path, "some content".as_bytes());
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content");
}

#[test]
fn append_file_exists() {
    let file_path = "./target/__test/ut/file_test/append_file/append_file_exists/file.txt";
    let mut result = append_file(file_path, "some content 1".as_bytes());
    assert!(result.is_ok());
    result = append_file(file_path, "some content 2".as_bytes());
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content 1some content 2");
}

#[test]
fn append_file_on_directory() {
    let file_path = "./target/__test/ut/file_test/append_file/append_file_on_directory";
    let mut result = directory::create(file_path);
    assert!(result.is_ok());
    result = append_file(file_path, "some content".as_bytes());
    assert!(result.is_err());
}

#[test]
fn modify_file_write_error() {
    let file_path = "./target/__test/ut/file_test/modify_file/modify_file_write_error";
    let result = modify_file(
        file_path,
        &move |_: &mut File| Err(io::Error::new(ErrorKind::Other, "test")),
        false,
    );
    assert!(result.is_err());
}

#[test]
fn read_text_file_not_exists() {
    let file_path =
        "./target/__test/ut/file_test/read_text_file/read_text_file_not_exists/file.txt";
    let result = read_text_file(file_path);

    assert!(result.is_err());
}

#[test]
fn read_text_file_exists() {
    let file_path = "./target/__test/ut/file_test/read_text_file/read_text_file_exists/file.txt";
    let result = write_text_file(file_path, "some content");
    assert!(result.is_ok());

    let text = read_text_file(file_path).unwrap();

    assert_eq!(text, "some content");
}

#[test]
fn read_file_not_exists() {
    let file_path = "./target/__test/ut/file_test/read_file/read_text_file_not_exists/file.txt";
    let result = read_file(file_path);

    assert!(result.is_err());
}

#[test]
fn read_file_exists() {
    let file_path = "./target/__test/ut/file_test/read_file/read_text_file_exists/file.txt";
    let result = write_text_file(file_path, "some content");
    assert!(result.is_ok());

    let data = read_file(file_path).unwrap();

    assert_eq!(str::from_utf8(&data).unwrap(), "some content");
}

#[test]
fn delete_exists() {
    let file_path = "./target/__test/ut/file_test/delete_file/delete_file_exists/file.txt";
    let mut result = ensure_exists(file_path);
    assert!(result.is_ok());

    let path = Path::new(file_path);
    assert!(path.exists());

    result = delete(file_path);
    assert!(result.is_ok());

    assert!(!path.exists());
}

#[test]
fn delete_not_exists() {
    let file_path = "./target/__test/ut/file_test/delete_file/delete_file_not_exists/file.txt";

    let path = Path::new(file_path);
    assert!(!path.exists());

    let result = delete(file_path);
    assert!(result.is_ok());

    assert!(!path.exists());
}

#[test]
fn delete_directory() {
    let file_path = "./target/__test/ut/file_test/delete_file/delete_directory/file.txt";
    let mut result = ensure_exists(file_path);
    assert!(result.is_ok());

    let path = Path::new(file_path);
    assert!(path.exists());

    result = delete("./target/__test/ut/file_test/delete_file/delete_directory");
    assert!(result.is_err());

    assert!(path.exists());
}

#[test]
fn delete_ignore_error_file_exists() {
    let file_path = "./target/__test/ut/file_test/delete_ignore_error/delete_file_exists/file.txt";
    let result = ensure_exists(file_path);
    assert!(result.is_ok());

    let path = Path::new(file_path);
    assert!(path.exists());

    let deleted = delete_ignore_error(file_path);
    assert!(deleted);

    assert!(!path.exists());
}

#[test]
fn delete_ignore_error_file_not_exists() {
    let file_path =
        "./target/__test/ut/file_test/delete_ignore_error/delete_file_not_exists/file.txt";

    let path = Path::new(file_path);
    assert!(!path.exists());

    let result = delete_ignore_error(file_path);
    assert!(result);

    assert!(!path.exists());
}

#[test]
fn delete_ignore_error_directory() {
    let file_path = "./target/__test/ut/file_test/delete_ignore_error/delete_directory/file.txt";
    let result = ensure_exists(file_path);
    assert!(result.is_ok());

    let path = Path::new(file_path);
    assert!(path.exists());

    let deleted =
        delete_ignore_error("./target/__test/ut/file_test/delete_ignore_error/delete_directory");
    assert!(!deleted);

    assert!(path.exists());
}
