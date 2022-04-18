/*!
Structured, streaming values

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or some text or binary format.
*/

#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, string, vec},
        core::{convert, fmt, mem, ops, result, str},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

mod data;
mod stream;
mod tag;
mod value;

pub mod result;

#[doc(inline)]
pub use self::{result::Error, stream::*, tag::*, value::*};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
