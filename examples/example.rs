use fsio::{directory, file, path};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;

fn main() {
    // file operations
    let mut result = file::ensure_exists("./target/__test/example/file_test/dir1/dir2/file.txt");
    assert!(result.is_ok());

    // create/append and read text files
    let mut file_path = "./target/__test/example/file_test/append_text_file/file.txt";
    result = file::write_text_file(file_path, "some content");
    assert!(result.is_ok());
    result = file::append_text_file(file_path, "\nmore content");
    assert!(result.is_ok());
    let mut text = file::read_text_file(file_path).unwrap();
    assert_eq!(text, "some content\nmore content");

    // create/append and read binary files
    file_path = "./target/__test/example/file_test/append_and_read_file_test/file.txt";
    result = file::write_file(file_path, "some content".as_bytes());
    assert!(result.is_ok());
    result = file::append_file(file_path, "\nmore content".as_bytes());
    assert!(result.is_ok());
    let data = file::read_file(file_path).unwrap();
    assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");

    // custom writing
    file_path = "./target/__test/file_test/modify_file/file.txt";
    result = file::modify_file(
        file_path,
        &move |file: &mut File| file.write_all("some content".as_bytes()),
        false,
    );
    assert!(result.is_ok());
    text = file::read_text_file(file_path).unwrap();
    assert_eq!(text, "some content");

    // delete file
    result = file::delete(file_path);
    assert!(result.is_ok());

    // directory operations
    result = directory::create("./target/__test/example/directory_test/dir1/dir2");
    assert!(result.is_ok());

    result = directory::create_parent("./target/__test/example/directory_test/dir1/files/file.txt");
    assert!(result.is_ok());

    // delete directory
    result = directory::delete("./target/__test/example/directory_test");
    assert!(result.is_ok());

    // basename and parent directory examples
    let basename = path::get_basename("./src/path/mod.rs");
    assert_eq!(basename.unwrap(), "mod.rs");

    let dirname = path::get_parent_directory("./src/path/mod.rs");
    assert_eq!(dirname.unwrap(), "./src/path");

    // canonicalize examples
    let path_obj = Path::new("./src/path/mod.rs");

    let path1 = path::canonicalize_as_string(&path_obj);
    let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");

    assert_eq!(path1.unwrap(), path2);
}
