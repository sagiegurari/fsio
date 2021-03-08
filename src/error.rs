//! # error
//!
//! The error structure and types.
//!

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::time::SystemTimeError;

#[cfg(test)]
#[path = "./error_test.rs"]
mod error_test;

#[derive(Debug)]
/// Holds the error information
pub enum FsIOError {
    /// Path already exist error type
    PathAlreadyExists(String),
    /// Not a file error type
    NotFile(String),
    /// IO error type
    IOError(String, Option<io::Error>),
    /// System time error type
    SystemTimeError(String, Option<SystemTimeError>),
}

impl Display for FsIOError {
    /// Formats the error using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::PathAlreadyExists(ref message) => write!(formatter, "{}", message),
            Self::NotFile(ref message) => write!(formatter, "{}", message),
            Self::IOError(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
            Self::SystemTimeError(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
        }
    }
}

impl Error for FsIOError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PathAlreadyExists(_) => None,
            Self::NotFile(_) => None,
            Self::IOError(_, error) => error.as_ref().map(|io_error| {
                let std_error: &dyn Error = io_error;
                std_error
            }),
            Self::SystemTimeError(_, error) => error.as_ref().map(|system_time_error| {
                let std_error: &dyn Error = system_time_error;
                std_error
            }),
        }
    }
}
