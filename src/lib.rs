#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use alloc::*;
    pub use core::*;
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

pub mod digits;
pub mod receiver;
pub mod source;
pub mod tag;
pub mod value;

mod for_all;
mod impls;

#[cfg(feature = "derive")]
pub use derive::*;

#[doc(inline)]
pub use self::{
    for_all::{for_all, ForAll},
    receiver::Receiver,
    source::{stream_to_end, Source},
    value::{stream, Value},
};

#[derive(Debug)]
pub struct Error;

impl From<std::fmt::Error> for Error {
    #[inline]
    fn from(_: std::fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
