use crate::Receiver;
use sval::source::{Next, TakeError, TryTakeError};

mod private {
    use sval::source::Next;

    use crate::receiver::Receiver;

    pub trait DispatchSource<'a> {
        fn dispatch_stream_resume<'b>(
            &mut self,
            receiver: &mut dyn Receiver<'b>,
        ) -> sval::Result<Next>
        where
            'a: 'b;

        fn dispatch_stream_to_end<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result
        where
            'a: 'b;
    }

    pub trait DispatchValueSource<
        'a,
        V: sval::SourceValue + ?Sized,
        R: sval::SourceValue + ?Sized = V,
    >: DispatchSource<'a>
    {
        fn dispatch_take(&mut self) -> sval::Result<&V>;

        fn dispatch_take_owned(&mut self) -> sval::Result<V::Owned>
        where
            V: sval::source::ToOwned,
            V::Owned: sval::SourceValue;

        fn dispatch_try_take_ref(&mut self) -> sval::Result<&'a R, sval::Result<&V>>;

        fn dispatch_try_take_owned(&mut self) -> sval::Result<V::Owned, sval::Result<&V>>
        where
            V: sval::source::ToOwned,
            V::Owned: sval::SourceValue;
    }

    pub trait EraseSource<'a> {
        fn erase_source(&mut self) -> crate::private::Erased<&mut dyn DispatchSource<'a>>;
    }

    pub trait EraseValueSource<'a, V: sval::SourceValue + ?Sized, R: sval::SourceValue + ?Sized> {
        fn erase_value_source(
            &mut self,
        ) -> crate::private::Erased<&mut dyn DispatchValueSource<'a, V, R>>;
    }
}

use sval::{Result, SourceValue};

pub trait Source<'a>: private::EraseSource<'a> {}

pub trait ValueSource<'a, V: sval::SourceValue + ?Sized, R: sval::SourceValue + ?Sized = V>:
    Source<'a> + private::EraseValueSource<'a, V, R>
{
}

impl<'a, S: sval::Source<'a>> Source<'a> for S {}

impl<'a, S: sval::Source<'a>> private::EraseSource<'a> for S {
    fn erase_source(&mut self) -> crate::private::Erased<&mut dyn private::DispatchSource<'a>> {
        crate::private::Erased(self)
    }
}

impl<'a, S: sval::Source<'a>> private::DispatchSource<'a> for S {
    fn dispatch_stream_resume<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result<Next>
    where
        'a: 'b,
    {
        self.stream_next(receiver)
    }

    fn dispatch_stream_to_end<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        self.stream_all(receiver)
    }
}

impl<
        'a,
        V: sval::SourceValue + ?Sized,
        R: sval::SourceValue + ?Sized,
        S: sval::SourceRef<'a, V, R>,
    > ValueSource<'a, V, R> for S
{
}

impl<
        'a,
        V: sval::SourceValue + ?Sized,
        R: sval::SourceValue + ?Sized,
        S: sval::SourceRef<'a, V, R>,
    > private::EraseValueSource<'a, V, R> for S
{
    fn erase_value_source(
        &mut self,
    ) -> crate::private::Erased<&mut dyn private::DispatchValueSource<'a, V, R>> {
        crate::private::Erased(self)
    }
}

impl<
        'a,
        V: sval::SourceValue + ?Sized,
        R: sval::SourceValue + ?Sized,
        S: sval::SourceRef<'a, V, R>,
    > private::DispatchValueSource<'a, V, R> for S
{
    fn dispatch_take(&mut self) -> Result<&V> {
        self.take().map_err(Into::into)
    }

    fn dispatch_take_owned(&mut self) -> Result<V::Owned>
    where
        V: sval::source::ToOwned,
        V::Owned: SourceValue,
    {
        self.take_owned().map_err(Into::into)
    }

    fn dispatch_try_take_ref(&mut self) -> Result<&'a R, Result<&V>> {
        self.try_take_ref()
            .map_err(|e| e.into_result().map_err(Into::into))
    }

    fn dispatch_try_take_owned(&mut self) -> Result<V::Owned, Result<&V>>
    where
        V: sval::source::ToOwned,
        V::Owned: SourceValue,
    {
        self.try_take_owned()
            .map_err(|e| e.into_result().map_err(Into::into))
    }
}

macro_rules! impl_source {
    ($($impl:tt)*) => {
        $($impl)* {
            fn stream_resume<'b, R: sval::Receiver<'b>>(&mut self, mut receiver: R) -> sval::Result<Stream>
            where
                'a: 'b,
            {
                self.erase_source().0.dispatch_stream_resume(&mut receiver)
            }

            fn stream_to_end<'b, R: sval::Receiver<'b>>(&mut self, mut receiver: R) -> sval::Result
            where
                'a: 'b,
            {
                self.erase_source().0.dispatch_stream_to_end(&mut receiver)
            }
        }
    }
}

macro_rules! impl_value_source {
    ({ $($s:tt)* } { $($vs:tt)* }) => {
        impl_source!($($s)*);

        $($vs)* {
            type Error = sval::Error;

            fn take(&mut self) -> Result<&V, TakeError<Self::Error>> {
                self.erase_value_source()
                    .0
                    .dispatch_take()
                    .map_err(TakeError::from_error)
            }

            fn take_owned(&mut self) -> Result<V::Owned, TakeError<Self::Error>>
            where
                V: sval::source::ToOwned,
                V::Owned: Value,
            {
                self.erase_value_source()
                    .0
                    .dispatch_take_owned()
                    .map_err(TakeError::from_error)
            }

            fn try_take_ref(&mut self) -> Result<&'a U, TryTakeError<&V, Self::Error>> {
                self.erase_value_source()
                    .0
                    .dispatch_try_take_ref()
                    .map_err(TryTakeError::from_result)
            }

            fn try_take_owned(&mut self) -> Result<V::Owned, TryTakeError<&V, Self::Error>>
            where
                V: sval::source::ToOwned,
                V::Owned: Value,
            {
                self.erase_value_source()
                    .0
                    .dispatch_try_take_owned()
                    .map_err(TryTakeError::from_result)
            }
        }
    }
}

impl_source!(impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + 'd);
impl_source!(impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + Send + 'd);
impl_source!(impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + Send + Sync + 'd);

impl_value_source!(
    {
        impl<'a, 'd, V: sval::SourceValue + ?Sized, U: sval::SourceValue + ?Sized> sval::Source<'a> for dyn ValueSource<'a, V, U> + 'd
    }
    {
        impl<'a, 'd, V: sval::SourceValue + ?Sized, U: sval::SourceValue + ?Sized> sval::SourceRef<'a, V, U> for dyn ValueSource<'a, V, U> + 'd
    }
);
impl_value_source!(
    {
        impl<'a, 'd, V: sval::SourceValue + ?Sized, U: sval::SourceValue + ?Sized> sval::Source<'a> for dyn ValueSource<'a, V, U> + Send + 'd
    }
    {
        impl<'a, 'd, V: sval::SourceValue + ?Sized, U: sval::SourceValue + ?Sized> sval::SourceRef<'a, V, U> for dyn ValueSource<'a, V, U> + Send + 'd
    }
);
impl_value_source!(
    {
        impl<'a, 'd, V: sval::SourceValue + ?Sized, U: sval::SourceValue + ?Sized> sval::Source<'a> for dyn ValueSource<'a, V, U> + Send + Sync + 'd
    }
    {
        impl<'a, 'd, V: sval::SourceValue + ?Sized, U: sval::SourceValue + ?Sized> sval::SourceRef<'a, V, U> for dyn ValueSource<'a, V, U> + Send + Sync + 'd
    }
);
