#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod stream;
mod value;

mod private {
    pub struct Erased<T>(pub(crate) T);
}

pub use self::{stream::Stream, value::Value};
