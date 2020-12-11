use crate::error::*;
use alloc::format;
use alloc::string::*;
use rcstring::CString;

#[inline]
pub fn as_cstring<V, T, F>(v: V, f: F) -> Result<T, Error>
where
    String: From<V>,
    F: FnOnce(CString<'_>) -> Result<T, Error>,
{
    let s: String = v.into();
    let string = format!("{}\0", s);
    f(CString::new(&string)?)
}
