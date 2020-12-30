use alloc::format;
use alloc::string::*;
use core::fmt::{self, Debug, Display, Formatter};
use rcstring;

use crate::util::from_cstring_raw;

pub enum Error {
    System(i32),
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

pub fn get_errno() -> libc::c_int {
    unsafe { *__errno() }
}

pub fn from_errno() -> Error {
    Error::System(get_errno())
}
