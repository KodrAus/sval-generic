use crate::{
    std::fmt::{self, Display},
    Receiver, Result, Value,
};

#[cfg(feature = "std")]
pub fn error<'a>(err: &'a (dyn ErrorImpl + 'static)) -> &'a Error {
    Error::new(err)
}

#[cfg(not(feature = "std"))]
trait ErrorImpl: Display {}

#[cfg(feature = "std")]
use crate::std::error::Error as ErrorImpl;

#[repr(transparent)]
pub struct Error(dyn ErrorImpl + 'static);

#[cfg(feature = "std")]
impl Error {
    pub fn new<'a>(err: &'a (dyn ErrorImpl + 'static)) -> &'a Error {
        // SAFETY: `Error` and `dyn Error + 'static` have the same ABI
        unsafe { &*(err as *const (dyn ErrorImpl + 'static) as *const Error) }
    }
}

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
