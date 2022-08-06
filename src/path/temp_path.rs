use crate::path::from_path::FromPath;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::iter;

#[cfg(windows)]
fn get_extra_path() -> Option<String> {
    let name = env!("CARGO_PKG_NAME").to_string();
    Some(name)
}

#[cfg(not(windows))]
fn get_extra_path() -> Option<String> {
    None
}

fn get_random_string() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect()
}

pub(crate) fn get(extension: &str) -> String {
    let name = env!("CARGO_PKG_NAME");
    let mut file_name = get_random_string();
    file_name.insert(0, '_');
    file_name.insert_str(0, name);

    let mut file_path = env::temp_dir();

    if let Some(extra_path) = get_extra_path() {
        file_path.push(extra_path);
    };

    file_path.push(file_name);
    file_path.set_extension(extension);

    FromPath::from_path(&file_path)
}
