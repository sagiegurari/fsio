use super::*;

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
