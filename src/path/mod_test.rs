use super::*;

use std::path::Path;

#[test]
fn canonicalize_as_string_valid() {
    let path = Path::new("./src/path/mod.rs");

    let path1 = canonicalize_as_string(&path);
    let path2 = canonicalize_as_string("./src/path/mod.rs");

    assert_eq!(path1.unwrap(), path2.unwrap());
}

#[test]
fn canonicalize_as_string_invalid() {
    let result = canonicalize_as_string("./src/path/mod.rs.2");

    assert!(result.is_err());
}

#[test]
fn canonicalize_or_valid() {
    let path_obj = Path::new("./src/path/mod.rs");

    let path1 = canonicalize_as_string(&path_obj);
    let path2 = canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");

    assert_eq!(path1.unwrap(), path2);
}

#[test]
fn canonicalize_or_invalid() {
    let result = canonicalize_or("./src/path/mod.rs.2", "/src/path/mod.rs.2");

    assert_eq!(result, "/src/path/mod.rs.2");
}

#[test]
fn get_basename_only_filename() {
    let result = get_basename("test.txt").unwrap();

    assert_eq!(result, "test.txt");
}

#[test]
fn get_basename_root_and_filename() {
    let result = get_basename("/test.txt").unwrap();

    assert_eq!(result, "test.txt");
}

#[test]
fn get_basename_only_dirname() {
    let result = get_basename("test").unwrap();

    assert_eq!(result, "test");
}

#[test]
fn get_basename_with_parent_directory() {
    let result = get_basename("test/file.txt").unwrap();

    assert_eq!(result, "file.txt");
}

#[test]
fn get_basename_with_relative_path_parent_directory() {
    let result = get_basename("./test/file.txt").unwrap();

    assert_eq!(result, "file.txt");
}

#[test]
fn get_basename_only_root() {
    let result = get_basename("/");

    assert!(result.is_none());
}

#[test]
fn get_parent_directory_only_filename() {
    let result = get_parent_directory("test.txt");

    assert!(result.is_none());
}

#[test]
fn get_parent_directory_root_and_filename() {
    let result = get_parent_directory("/test.txt").unwrap();

    assert_eq!(result, "/");
}

#[test]
fn get_parent_directory_only_dirname() {
    let result = get_parent_directory("test");

    assert!(result.is_none());
}

#[test]
fn get_parent_directory_with_parent_directory() {
    let result = get_parent_directory("test/file.txt").unwrap();

    assert_eq!(result, "test");
}

#[test]
fn get_parent_directory_with_relative_path_parent_directory() {
    let result = get_parent_directory("./test/file.txt").unwrap();

    assert_eq!(result, "./test");
}

#[test]
fn get_parent_directory_only_root() {
    let result = get_parent_directory("/");

    assert!(result.is_none());
}

#[test]
fn get_last_modified_time_file() {
    let time = get_last_modified_time("./Cargo.toml").unwrap();
    assert!(time > 0);
}

#[test]
fn get_last_modified_time_directory() {
    let time = get_last_modified_time("./src").unwrap();
    assert!(time > 0);
}

#[test]
fn get_last_modified_time_not_found() {
    let result = get_last_modified_time("./badfile");
    assert!(result.is_err());
}

#[test]
#[cfg(feature = "temp-path")]
fn get_temporary_file_path_valid() {
    let temp_file = get_temporary_file_path("txt");

    assert!(temp_file.ends_with(".txt"));
    assert!(temp_file.contains("fsio_"));
    if cfg!(windows) {
        assert!(temp_file.contains("/fsio/fsio_"));
    }
}
