use crate::std::{fmt, str};

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl From<fmt::Error> for Error {
    #[inline]
    fn from(_: fmt::Error) -> Error {
        Error
    }
}

impl From<Error> for fmt::Error {
    #[inline]
    fn from(_: Error) -> fmt::Error {
        fmt::Error
    }
}

impl From<str::Utf8Error> for Error {
    #[inline]
    fn from(_: str::Utf8Error) -> Error {
        Error
    }
}

pub fn unsupported() -> crate::Result {
    Err(crate::Error)
}
