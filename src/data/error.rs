use crate::{
    std::fmt::{self, Display},
    Receiver, Result, Value,
};

#[cfg(feature = "std")]
pub fn error<'a>(err: &'a (dyn Inner + 'static)) -> &'a Error {
    Error::new(err)
}

#[cfg(not(feature = "std"))]
trait Inner: Display {}

#[cfg(feature = "std")]
use crate::std::error::Error as Inner;

#[repr(transparent)]
pub struct Error(dyn Inner + 'static);

impl Value for Error {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.error(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::{
        source::{self, ValueSource},
        std::{error, fmt, io},
        Receiver, Result, Value,
    };

    impl Error {
        pub fn new<'a>(err: &'a (dyn error::Error + 'static)) -> &'a Error {
            // SAFETY: `Error` and `dyn Error + 'static` have the same ABI
            unsafe { &*(err as *const (dyn Inner + 'static) as *const Error) }
        }
    }

    impl Value for dyn error::Error + 'static {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> ValueSource<'a, Error> for &'a (dyn error::Error + 'static) {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Error, source::TakeError<Self::Error>> {
            Ok(Error::new(*self))
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a Error, source::TryTakeError<&Error, Self::Error>> {
            Ok(Error::new(*self))
        }
    }

    impl Value for io::Error {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> ValueSource<'a, Error> for &'a io::Error {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Error, source::TakeError<Self::Error>> {
            Ok(Error::new(*self))
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a Error, source::TryTakeError<&Error, Self::Error>> {
            Ok(Error::new(*self))
        }
    }

    impl Value for fmt::Error {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> ValueSource<'a, Error> for &'a fmt::Error {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Error, source::TakeError<Self::Error>> {
            Ok(Error::new(*self))
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a Error, source::TryTakeError<&Error, Self::Error>> {
            Ok(Error::new(*self))
        }
    }
}
