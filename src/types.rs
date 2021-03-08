//! # types
//!
//! Common types.
//!

use crate::error::FsIOError;

/// Result aliasing for project-wide error type.
pub type FsIOResult<T> = Result<T, FsIOError>;
