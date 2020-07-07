use super::*;

#[test]
fn display_error_path_already_exists() {
    let error = FsIOError {
        info: ErrorInfo::PathAlreadyExists("test".to_string()),
    };
    println!("{}", error);
}

#[test]
fn display_error_not_file() {
    let error = FsIOError {
        info: ErrorInfo::NotFile("test".to_string()),
    };
    println!("{}", error);
}

#[test]
fn display_error_io_error() {
    let error = FsIOError {
        info: ErrorInfo::IOError("test".to_string(), None),
    };
    println!("{}", error);
}

#[test]
fn display_error_system_time_error() {
    let error = FsIOError {
        info: ErrorInfo::SystemTimeError("test".to_string(), None),
    };
    println!("{}", error);
}
