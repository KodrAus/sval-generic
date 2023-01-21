/*!
Buffering support for `sval`.

This crate provides the [`ValueBuf`] type, which can buffer a flat
stream of data into a tree of borrowed values.
*/

#![no_std]

#[cfg(not(feature = "alloc"))]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
extern crate core;

#[cfg(feature = "alloc")]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, marker, mem, ops, result, str},
    };
}

mod fragments;
mod value;

pub use self::{fragments::*, value::*};
