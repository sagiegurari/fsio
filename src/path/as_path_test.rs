use super::*;

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
