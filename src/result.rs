use crate::std::{fmt, str};

#[derive(Debug)]
pub struct Error(());

impl Error {
    pub fn unsupported() -> Self {
        Error(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl From<fmt::Error> for Error {
    #[inline]
    fn from(_: fmt::Error) -> Error {
        Error(())
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
        Error(())
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::{error, io};

    impl error::Error for Error {}

    impl From<io::Error> for Error {
        #[inline]
        fn from(_: io::Error) -> Error {
            Error(())
        }
    }

    impl From<Error> for io::Error {
        fn from(e: Error) -> io::Error {
            io::Error::new(io::ErrorKind::Other, e)
        }
    }
}

pub fn unsupported<T>() -> crate::Result<T> {
    Err(Error::unsupported())
}
