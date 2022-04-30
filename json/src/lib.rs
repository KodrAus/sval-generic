#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod from_slice;
mod to_fmt;
pub use self::{
    from_slice::{slice, JsonSlice, JsonSliceReader},
    to_fmt::{to_fmt, Formatter},
};

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::to_string;

mod from_slice_co;
pub use self::from_slice_co::JsonSliceCoReader;
