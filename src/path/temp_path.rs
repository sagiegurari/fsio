use crate::path::from_path::FromPath;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::iter;

#[cfg(windows)]
fn get_additional_temp_path() -> Option<String> {
    None
}

#[cfg(not(windows))]
fn get_additional_temp_path() -> Option<String> {
    let random_string = get_random_string();
    let mut additional_path = env!("CARGO_PKG_NAME").to_string();
    additional_path.push('_');
    additional_path.push_str(&random_string);
    Some(additional_path.to_string())
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

    let file_name = get_random_string();
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
