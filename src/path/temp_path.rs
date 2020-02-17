use crate::path::from_path::FromPath;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::iter;

#[cfg(not(windows))]
use users::get_current_username;

#[cfg(windows)]
fn get_additional_temp_path() -> Option<String> {
    None
}

#[cfg(not(windows))]
fn get_additional_temp_path() -> Option<String> {
    let username = get_current_username();

    match username {
        Some(os_value) => match os_value.into_string() {
            Ok(value) => Some(value),
            Err(_) => None,
        },
        None => None,
    }
}

pub(crate) fn get(extension: &str) -> String {
    let name = env!("CARGO_PKG_NAME");

    let mut rng = thread_rng();
    let file_name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(10)
        .collect();

    let mut file_path = env::temp_dir();

    match get_additional_temp_path() {
        Some(additional_path) => file_path.push(additional_path),
        None => {}
    };

    file_path.push(name);

    file_path.push(file_name);
    file_path.set_extension(extension);

    FromPath::from_path(&file_path)
}
