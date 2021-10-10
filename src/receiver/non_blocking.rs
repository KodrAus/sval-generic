use crate::{Result, Source, Value};

#[async_trait]
pub trait AsyncReceiver<'a>: Send {
    async fn blocking<'v: 'a, V: Source<'v> + Send>(&mut self, value: V) -> Result;

    async fn value<'v: 'a, V: Value + Sync + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        value.stream_non_blocking(self).await
    }
}

#[async_trait]
impl<'a, 'b, R: AsyncReceiver<'a> + ?Sized> AsyncReceiver<'a> for &'b mut R {
    async fn blocking<'v: 'a, V: Source<'v> + Send>(&mut self, value: V) -> Result {
        (**self).blocking(value).await
    }

    async fn value<'v: 'a, V: Value + Sync + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        (**self).value(value).await
    }
}
