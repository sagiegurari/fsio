use fsio::path;
use fsio::path::as_path::AsPath;
use fsio::path::from_path::FromPath;
use std::path::{Path, PathBuf};

#[test]
fn as_path_str() {
    let path = Path::new("./test/file.txt");
    let as_path = "./test/file.txt".as_path();

    assert_eq!(path, as_path);
}

#[test]
fn as_path_string() {
    let path = Path::new("./test/file.txt");

    let path_string = "./test/file.txt".to_string();
    let as_path = path_string.as_path();

    assert_eq!(path, as_path);
}

#[test]
fn as_path_path() {
    let path = Path::new("./test/file.txt");
    let as_path = path.as_path();

    assert_eq!(path, as_path);
}

#[test]
fn as_path_path_buf() {
    let path = Path::new("./test/file.txt");

    let mut path_buf = PathBuf::new();
    path_buf.push(".");
    path_buf.push("test");
    path_buf.push("file.txt");
    let as_path = path_buf.as_path();

    assert_eq!(path, as_path);
}

#[test]
fn from_path_string() {
    let path = Path::new("./test/file.txt");
    let from_path: String = FromPath::from_path(&path);

    assert_eq!("./test/file.txt", from_path);
}

#[test]
fn from_path_path_buf() {
    let path = Path::new("./test/file.txt");
    let from_path: PathBuf = FromPath::from_path(&path);

    assert_eq!(path.to_path_buf(), from_path);
}

#[test]
fn canonicalize_as_string_test() {
    let path_obj = Path::new("./src/path/mod.rs");

    let path1 = path::canonicalize_as_string(&path_obj);
    let path2 = path::canonicalize_as_string("./src/path/mod.rs");

    assert_eq!(path1.unwrap(), path2.unwrap());
}

#[test]
fn canonicalize_or_valid() {
    let path_obj = Path::new("./src/path/mod.rs");

    let path1 = path::canonicalize_as_string(&path_obj);
    let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");

    assert_eq!(path1.unwrap(), path2);
}

#[test]
fn get_basename_test() {
    let basename = path::get_basename("./src/path/mod.rs");

    assert_eq!(basename.unwrap(), "mod.rs");
}

#[test]
fn get_parent_directory_test() {
    let dirname = path::get_parent_directory("./src/path/mod.rs");

    assert_eq!(dirname.unwrap(), "./src/path");
}

#[test]
fn get_last_modified_time_test() {
    let time = path::get_last_modified_time("./src/path/mod.rs").unwrap();

    assert!(time > 0);
}
