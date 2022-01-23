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
    source::{Source, ValueSource},
    value::Value,
};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

fn lol<'a>(v: &'a impl Value, mut r: impl Receiver<'a>) {
    r.tagged_map_begin(data::tag("A").with_content_hint(data::tag::ContentHint::Struct))
        .unwrap();

    r.map_field_entry("a", v).unwrap();
    r.map_field_entry(
        "b",
        data::tagged("ts", "1985-04-12T23:20:50.52Z")
            .with_content_hint(data::tag::ContentHint::RFC3339DateTime),
    )
    .unwrap();

    r.tagged_map_end().unwrap();
}
