use std::{borrow::ToOwned, fmt};

use crate::{
    source::{Impossible, ToRefError, ToValueError},
    AsyncReceiver, Error, Result, Value,
};

#[async_trait]
pub trait AsyncSource<'a>: Send {
    async fn stream<'b, R: AsyncReceiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b;
}

#[async_trait]
impl<'a, 'b, T: AsyncSource<'a> + ?Sized> AsyncSource<'a> for &'b mut T {
    async fn stream<'c, S: AsyncReceiver<'c>>(&mut self, stream: S) -> Result
    where
        'a: 'c,
    {
        (**self).stream(stream).await
    }
}

#[async_trait]
pub trait AsyncValueSource<'a, T: Value + Sync + ?Sized>: AsyncSource<'a> {
    type Error: Into<Error> + fmt::Debug;

    async fn value(&mut self) -> Result<&T, ToValueError<Self::Error>>;

    async fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Err(ToRefError::from_result(
            self.value().await.map_err(|e| e.into_inner()),
        ))
    }

    async fn value_owned(&mut self) -> Result<T::Owned, ToValueError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value + Send,
    {
        self.value().await.map(ToOwned::to_owned)
    }
}

#[async_trait]
impl<'a, 'b, T: Value + Sync + ?Sized, S: AsyncValueSource<'a, T> + ?Sized> AsyncValueSource<'a, T>
    for &'b mut S
{
    type Error = S::Error;

    async fn value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        (**self).value().await
    }

    async fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        (**self).value_ref().await
    }

    async fn value_owned(&mut self) -> Result<T::Owned, ToValueError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value + Send,
    {
        (**self).value_owned().await
    }
}

#[async_trait]
impl<'a, T: Value + Sync + ?Sized> AsyncSource<'a> for &'a T {
    async fn stream<'b, R: AsyncReceiver<'b> + Send>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.value(*self).await
    }
}

#[async_trait]
impl<'a, T: Value + Sync + ?Sized> AsyncValueSource<'a, T> for &'a T {
    type Error = Impossible;

    async fn value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        Ok(self)
    }

    async fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Ok(self)
    }

    async fn value_owned(&mut self) -> Result<T::Owned, ToValueError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        Ok(self.to_owned())
    }
}
