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
    A tag for values that are already in a JSON compatible form.

    For strings, that means they either don't need escaping or are already escaped.
    For numbers, that means they're already in a JSON compatible format.
    */
    pub const JSON_NATIVE: sval::Tag<'static> = sval::Tag::new("svaljsonnat");
}

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::to_string;
