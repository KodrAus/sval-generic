use crate::std::fmt;

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

pub fn unsupported() -> crate::Result {
    Err(crate::Error)
}
