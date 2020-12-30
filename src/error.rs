//! Error

use alloc::format;
use alloc::string::*;
use core::fmt::{self, Debug, Display, Formatter};
use rcstring;

use crate::util::from_cstring_raw;

/// Represents a runtime error.
pub enum Error {
    /// Represents a runtime error which comes from the underlying platform
    /// (PROS, FreeRTOS, newlib, etc.). It wraps an `errno` value (i.e., system
    /// error code).
    System(i32),
    /// Represents a runtime error which comes from within Rust. It wraps an
    /// error string.
    Custom(String),
}

impl From<rcstring::Error> for Error {
    fn from(err: rcstring::Error) -> Self {
        Error::Custom(format!("{:?}", err))
    }
}

impl<T> From<Error> for Result<T, Error> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::System(n) => write!(f, "System({}) [{}]", n, unsafe {
                from_cstring_raw(libc::strerror(*n))
            }),
            Error::Custom(s) => write!(f, "Custom({:?})", s),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::System(n) => Display::fmt(unsafe { &from_cstring_raw(libc::strerror(*n)) }, f),
            Error::Custom(s) => Display::fmt(s, f),
        }
    }
}

// Need to manually declare until https://github.com/rust-lang/libc/issues/1995 is resolved.
extern "C" {
    fn __errno() -> *mut i32;
}

/// Gets the value of `errno` for the current task.
pub fn get_errno() -> libc::c_int {
    unsafe { *__errno() }
}

/// Generates an [`Error`] object from the value of `errno` for the current
/// task.
pub fn from_errno() -> Error {
    Error::System(get_errno())
}
