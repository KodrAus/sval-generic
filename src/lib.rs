#![cfg_attr(not(test), no_std)]
#![feature(min_specialization)] // Used for optional internal private specialization

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
        core::{fmt, mem, ops, result, str},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

pub mod data;
mod receiver;
pub mod source;

pub mod error;
mod for_all;

#[doc(inline)]
pub use self::{
    error::Error,
    for_all::{for_all, ForAll},
    receiver::Receiver,
    source::{Source, SourceRef, SourceValue},
};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
