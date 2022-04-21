#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod writer;

mod to_debug;
mod to_fmt;

pub use self::{
    to_debug::{debug, Debug},
    to_fmt::{to_fmt, Formatter},
};

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::to_string;
