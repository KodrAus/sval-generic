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
        alloc::{borrow, string, vec},
        core::{fmt, mem, ops, result},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

pub mod data;
pub mod receiver;
pub mod source;
pub mod value;

mod error;
mod for_all;

#[cfg(feature = "derive")]
pub use derive::*;

#[doc(inline)]
pub use self::{
    error::Error,
    for_all::{for_all, ForAll},
    receiver::Receiver,
    source::{stream_to_end, Source},
    value::{stream, Value},
};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
