#[cfg(feature = "std")]
mod std_support {
    use crate::{
        data,
        source::{self, ValueSource},
        std::{error::Error, fmt, io},
        Receiver, Result, Value,
    };

    impl Value for dyn Error + 'static {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> ValueSource<'a, data::Error> for &'a (dyn Error + 'static) {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&data::Error, source::TakeError<Self::Error>> {
            Ok(data::Error::new(*self))
        }

        #[inline]
        fn take_ref(
            &mut self,
        ) -> Result<&'a data::Error, source::TakeRefError<&data::Error, Self::Error>> {
            Ok(data::Error::new(*self))
        }
    }

    impl Value for io::Error {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> ValueSource<'a, data::Error> for &'a io::Error {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&data::Error, source::TakeError<Self::Error>> {
            Ok(data::Error::new(*self))
        }

        #[inline]
        fn take_ref(
            &mut self,
        ) -> Result<&'a data::Error, source::TakeRefError<&data::Error, Self::Error>> {
            Ok(data::Error::new(*self))
        }
    }

    impl Value for fmt::Error {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.error(self)
        }
    }

    impl<'a> ValueSource<'a, data::Error> for &'a fmt::Error {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&data::Error, source::TakeError<Self::Error>> {
            Ok(data::Error::new(*self))
        }

        #[inline]
        fn take_ref(
            &mut self,
        ) -> Result<&'a data::Error, source::TakeRefError<&data::Error, Self::Error>> {
            Ok(data::Error::new(*self))
        }
    }
}
