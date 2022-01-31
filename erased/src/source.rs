use crate::Receiver;
use sval::source::{Stream, TakeError, TryTakeError};

mod private {
    use sval::source::Stream;

    use crate::receiver::Receiver;

    pub trait DispatchSource<'a> {
        fn dispatch_stream_resume<'b>(
            &mut self,
            receiver: &mut dyn Receiver<'b>,
        ) -> sval::Result<Stream>
        where
            'a: 'b;

        fn dispatch_stream_to_end<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result
        where
            'a: 'b;
    }

    pub trait DispatchValueSource<'a, V: sval::Value + ?Sized, R: sval::Value + ?Sized = V>:
        DispatchSource<'a>
    {
        fn dispatch_take(&mut self) -> sval::Result<&V>;

        fn dispatch_take_owned(&mut self) -> sval::Result<V::Owned>
        where
            V: sval::source::ToOwned,
            V::Owned: sval::Value;

        fn dispatch_try_take_ref(&mut self) -> sval::Result<&'a R, sval::Result<&V>>;

        fn dispatch_try_take_owned(&mut self) -> sval::Result<V::Owned, sval::Result<&V>>
        where
            V: sval::source::ToOwned,
            V::Owned: sval::Value;
    }

    pub trait EraseSource<'a> {
        fn erase_source(&mut self) -> crate::private::Erased<&mut dyn DispatchSource<'a>>;
    }

    pub trait EraseValueSource<'a, V: sval::Value + ?Sized, R: sval::Value + ?Sized> {
        fn erase_value_source(
            &mut self,
        ) -> crate::private::Erased<&mut dyn DispatchValueSource<'a, V, R>>;
    }
}

use sval::{Result, Value};

pub trait Source<'a>: private::EraseSource<'a> {}

pub trait ValueSource<'a, V: sval::Value + ?Sized, R: sval::Value + ?Sized = V>:
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
    fn dispatch_stream_resume<'b>(
        &mut self,
        receiver: &mut dyn Receiver<'b>,
    ) -> sval::Result<Stream>
    where
        'a: 'b,
    {
        self.stream_resume(receiver)
    }

    fn dispatch_stream_to_end<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        self.stream_to_end(receiver)
    }
}

impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + 'd {
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

impl<'a, V: sval::Value + ?Sized, R: sval::Value + ?Sized, S: sval::ValueSource<'a, V, R>>
    ValueSource<'a, V, R> for S
{
}

impl<'a, V: sval::Value + ?Sized, R: sval::Value + ?Sized, S: sval::ValueSource<'a, V, R>>
    private::EraseValueSource<'a, V, R> for S
{
    fn erase_value_source(
        &mut self,
    ) -> crate::private::Erased<&mut dyn private::DispatchValueSource<'a, V, R>> {
        crate::private::Erased(self)
    }
}

impl<'a, V: sval::Value + ?Sized, R: sval::Value + ?Sized, S: sval::ValueSource<'a, V, R>>
    private::DispatchValueSource<'a, V, R> for S
{
    fn dispatch_take(&mut self) -> Result<&V> {
        self.take().map_err(Into::into)
    }

    fn dispatch_take_owned(&mut self) -> Result<V::Owned>
    where
        V: sval::source::ToOwned,
        V::Owned: Value,
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
        V::Owned: Value,
    {
        self.try_take_owned()
            .map_err(|e| e.into_result().map_err(Into::into))
    }
}

impl<'a, 'd, V: sval::Value + ?Sized, U: sval::Value + ?Sized> sval::Source<'a>
    for dyn ValueSource<'a, V, U> + 'd
{
    fn stream_resume<'b, R: sval::Receiver<'b>>(&mut self, mut receiver: R) -> sval::Result<Stream>
    where
        'a: 'b,
    {
        self.erase_value_source()
            .0
            .dispatch_stream_resume(&mut receiver)
    }

    fn stream_to_end<'b, R: sval::Receiver<'b>>(&mut self, mut receiver: R) -> sval::Result
    where
        'a: 'b,
    {
        self.erase_value_source()
            .0
            .dispatch_stream_to_end(&mut receiver)
    }
}

impl<'a, 'd, V: sval::Value + ?Sized, R: sval::Value + ?Sized> sval::ValueSource<'a, V, R>
    for dyn ValueSource<'a, V, R> + 'd
{
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

    fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&V, Self::Error>> {
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
