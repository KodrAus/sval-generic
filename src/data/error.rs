use crate::{
    std::fmt::{self, Debug, Display},
    Receiver, Result, SourceValue,
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

impl SourceValue for Error {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.error(self)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(not(feature = "std"))]
        {
            Display::fmt(&self.0, f)
        }

        #[cfg(feature = "std")]
        {
            Debug::fmt(&self.0, f)
        }
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
        source, source_ref,
        std::{error, fmt, io},
        Receiver, Result, SourceRef, SourceValue,
    };

    impl Error {
        pub fn new<'a>(err: &'a (dyn error::Error + 'static)) -> &'a Error {
            // SAFETY: `Error` and `dyn Error + 'static` have the same ABI
            unsafe { &*(err as *const (dyn Inner + 'static) as *const Error) }
        }
    }

    impl SourceValue for dyn error::Error + 'static {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> SourceRef<'a, Error> for &'a (dyn error::Error + 'static) {
        type Error = source_ref::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Error, source_ref::TakeError<Self::Error>> {
            Ok(Error::new(*self))
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a Error, source_ref::TryTakeError<&Error, Self::Error>> {
            Ok(Error::new(*self))
        }
    }

    impl SourceValue for io::Error {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> SourceRef<'a, Error> for &'a io::Error {
        type Error = source_ref::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Error, source_ref::TakeError<Self::Error>> {
            Ok(Error::new(*self))
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a Error, source_ref::TryTakeError<&Error, Self::Error>> {
            Ok(Error::new(*self))
        }
    }

    impl SourceValue for fmt::Error {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> SourceRef<'a, Error> for &'a fmt::Error {
        type Error = source_ref::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Error, source_ref::TakeError<Self::Error>> {
            Ok(Error::new(*self))
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a Error, source_ref::TryTakeError<&Error, Self::Error>> {
            Ok(Error::new(*self))
        }
    }
}
