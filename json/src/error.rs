use core::fmt;

#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Generic,
    Fmt(fmt::Error),
    #[cfg(feature = "std")]
    IO(std::io::Error),
    InvalidKey,
}

impl Error {
    pub(crate) fn generic() -> Self {
        Error {
            kind: ErrorKind::Generic,
        }
    }

    pub(crate) fn invalid_key() -> Self {
        Error {
            kind: ErrorKind::InvalidKey,
        }
    }
}
