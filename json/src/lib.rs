#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod from_slice;
mod to_fmt;
pub use self::{
    from_slice::{slice, JsonSlice, JsonSliceReader},
    to_fmt::{to_fmt, Formatter},
};

pub mod tags {
    /**
    A tag from strings that are either already escaped in JSON, or that don't need any escaping.
    */
    pub const JSON_STRING: sval::Tag<'static> = sval::Tag::new("svaljsonstr");
}

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::to_string;
