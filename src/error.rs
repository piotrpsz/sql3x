#![allow(unused)]
use std::ffi::CStr;
use std::io;
use std::io::Error as IoError;
use std::io::ErrorKind;
use serde_json::Error as JsonError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub kind: Option<ErrorKind>
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error { code: -1, message: err.to_string(), kind: None }
    }   
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error {
            code: -1,
            message: err.to_string(),
            kind: err.io_error_kind()
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error { code: err.raw_os_error().unwrap(), message: err.to_string(), kind: Some(err.kind()) }
    }
}

pub fn from_errno() -> Error {
    unsafe {
        let errno = io::Error::last_os_error().raw_os_error().unwrap();
        let message = libc::strerror(errno);
        let cstr = CStr::from_ptr(message);
        let message = cstr.to_str().unwrap().to_string();
        let kind = io::Error::last_os_error().kind();
        Error { code: errno, message, kind: Some(kind) }
    }
}
