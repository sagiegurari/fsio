//! # error
//!
//! The error structure and types.
//!

use std::fmt;
use std::fmt::Display;
use std::io;

#[cfg(test)]
#[path = "./error_test.rs"]
mod error_test;

#[derive(Debug)]
/// Holds the error information
pub enum ErrorInfo {
    /// Error Info Type
    PathAlreadyExists(String),
    /// Error Info Type
    NotFile(String),
    /// Error Info Type
    IOError(String, Option<io::Error>),
}

#[derive(Debug)]
/// Error struct
pub struct FsIOError {
    /// Holds the error information
    pub info: ErrorInfo,
}

impl Display for FsIOError {
    /// Formats the error using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::PathAlreadyExists(ref message) => write!(formatter, "{}", message),
            ErrorInfo::NotFile(ref message) => write!(formatter, "{}", message),
            ErrorInfo::IOError(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
        }
    }
}
