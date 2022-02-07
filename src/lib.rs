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
        core::{fmt, mem, ops, result, str},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

pub mod data;
mod receiver;
mod source;
mod value;

pub mod error;

#[doc(inline)]
pub use self::{error::Error, receiver::*, source::*, value::*};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
