use super::*;

#[test]
fn display_error_path_already_exists() {
    let error = FsIOError::PathAlreadyExists("test".to_string());
    println!("{}", error);
}

#[test]
fn display_error_not_file() {
    let error = FsIOError::NotFile("test".to_string());
    println!("{}", error);
}

#[test]
fn display_error_io_error() {
    let error = FsIOError::IOError("test".to_string(), None);
    println!("{}", error);
}

#[test]
fn display_error_system_time_error() {
    let error = FsIOError::SystemTimeError("test".to_string(), None);
    println!("{}", error);
}
