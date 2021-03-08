# fsio

[![crates.io](https://img.shields.io/crates/v/fsio.svg)](https://crates.io/crates/fsio) [![CI](https://github.com/sagiegurari/fsio/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/fsio/actions) [![codecov](https://codecov.io/gh/sagiegurari/fsio/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/fsio)<br>
[![license](https://img.shields.io/crates/l/fsio.svg)](https://github.com/sagiegurari/fsio/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/fsio.svg)](https://libraries.io/cargo/fsio) [![Documentation](https://docs.rs/fsio/badge.svg)](https://docs.rs/crate/fsio/) [![downloads](https://img.shields.io/crates/d/fsio.svg)](https://crates.io/crates/fsio)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> File System and Path utility functions.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/fsio/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This crate contains utility functions for path, file and directory handling.

<a name="usage"></a>
## Usage
There are multiple main modules for fsio:

* fsio::path - Holds path related functions and traits. They do not directly modify the file system.
* fsio::file - File utility functions such as read_file, write_file, ...
* fsio::directory - Directory specific utility functions.

### Examples

<!--{ "examples/example.rs" | lines: 1 | code: rust }-->
```rust
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

    // get last modified time
    let time = path::get_last_modified_time("./src/path/mod.rs").unwrap();
    assert!(time > 0);
}
```
<!--{ end }-->

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
fsio = "^0.2.0"
```

If you need access to temporary file paths, enable the **temp-path** feature as follows:

```ini
[dependencies]
fsio = { version = "*", features = ["temp-path"] }
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/fsio/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
