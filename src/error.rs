use alloc::format;
use alloc::string::*;
use rcstring;

#[derive(Debug)]
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
