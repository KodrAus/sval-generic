#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod fmt;
mod slice;
pub use self::{
    fmt::{to_fmt, Formatter},
    slice::{JsonSlice, JsonSliceReader},
};

#[cfg(feature = "alloc")]
mod alloc_support;

#[cfg(feature = "alloc")]
pub use self::alloc_support::to_string;
