#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod receiver;
mod source;
mod value;

mod private {
    pub struct Erased<T>(pub(crate) T);
}

pub use self::{receiver::Receiver, source::Source, value::Value};

// TODO: Dynamic<&dyn Value> / Dynamic<&mut dyn Source>
// That way we can correctly convert dynamic data with necessary state
