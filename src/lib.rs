mod for_all;
mod impls;
pub mod stream;
pub mod value;
mod value_ref;

pub mod erased;
pub mod serde;

pub use self::{stream::Stream, value::Value};

#[derive(Debug)]
pub struct Error;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream<'a>(s: impl Stream<'a>, v: impl stream::UnknownValueRef<'a>) -> Result {
    v.stream(s)
}
