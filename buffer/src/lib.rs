#![no_std]

#[cfg(not(feature = "alloc"))]
#[macro_use]
extern crate core as std;

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;
#[cfg(feature = "alloc")]
#[macro_use]
extern crate core;

#[cfg(feature = "alloc")]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, mem, ops, result, str},
    };
}

mod fragments;

pub use self::fragments::*;

#[cfg(feature = "alloc")]
mod value;

#[cfg(feature = "alloc")]
pub use self::value::*;
