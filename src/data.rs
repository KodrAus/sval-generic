pub(crate) mod binary;
pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod optional;
pub(crate) mod seq;
pub(crate) mod text;

use crate::{
    std::{
        borrow::Borrow,
        fmt,
        hash::{Hash, Hasher},
        ops::Deref,
    },
    Result, Stream, Value,
};

pub use self::{binary::*, text::*};

pub mod tags {
    use super::Tag;

    /**
    A tag for a value that represents the `Some` variant of a Rust `Option`.
    */
    pub const RUST_OPTION_SOME: Tag<'static> = Tag::new("rsome");

    /**
    A tag for a value that represents the `None` variant of a Rust `Option`.
    */
    pub const RUST_OPTION_NONE: Tag<'static> = Tag::new("rnone");

    /**
    A tag for Rust's `()` type.

    This tag is applied to `null`s to indicate that they're a unit value.
    */
    pub const RUST_UNIT: Tag<'static> = Tag::new("r()");

    /**
    A tag for arbitrary-precision decimal numbers.
    */
    pub const NUMBER: Tag<'static> = Tag::new("svalnum");

    /**
    A tag for values that have a constant size.
    */
    pub const CONSTANT_SIZE: Tag<'static> = Tag::new("svalcs");
}

/**
A textual label for some value.
*/
#[derive(Clone, Copy)]
pub struct Label<'computed> {
    value_computed: &'computed str,
    value_static: Option<&'static str>,
}

impl<'computed> Label<'computed> {
    /**
    Create a new label from a static static value.

    For labels that can't satisfy the `'static` lifetime, use [`Label::computed`].
    */
    pub const fn new(label: &'static str) -> Self {
        Label {
            value_computed: label,
            value_static: Some(label),
        }
    }

    /**
    Create a new label from a string value.
    */
    pub const fn computed(label: &'computed str) -> Self {
        Label {
            value_computed: label,
            value_static: None,
        }
    }

    /**
    Get the value of the label as a string.
    */
    pub const fn get(&self) -> &'computed str {
        self.value_computed
    }

    /**
    Try get the value of the label as a static string.

    For labels that were created over computed data this method will return `None`.
    */
    pub const fn try_get_static(&self) -> Option<&'static str> {
        self.value_static
    }
}

impl<'a> Deref for Label<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value_computed
    }
}

impl<'a, 'b> PartialEq<Label<'b>> for Label<'a> {
    fn eq(&self, other: &Label<'b>) -> bool {
        self.value_computed == other.value_computed
    }
}

impl<'a> Eq for Label<'a> {}

impl<'a> Hash for Label<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value_computed.hash(state)
    }
}

impl<'a> Borrow<str> for Label<'a> {
    fn borrow(&self) -> &str {
        self.value_computed
    }
}

impl<'a> fmt::Debug for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value_computed.fmt(f)
    }
}

/**
A type tag for a value.

Tags are additional hints that a stream may use to interpret a value differently,
or to avoid some unnecessary work.
*/
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Tag<'a> {
    id: u64,
    data: &'a str,
}

impl<'a> Tag<'a> {
    /**
    Create a new tag with a given value.
    */
    pub const fn new(data: &'a str) -> Self {
        // Fast, non-cryptographic hash used by rustc and Firefox.
        // Adapted from: https://github.com/rust-lang/rustc-hash/blob/master/src/lib.rs to work in CTFE
        //
        // We use hashes for quick tag comparison, if they collide then we'll compare the full value
        const fn compute_id(bytes: &[u8]) -> u64 {
            // Copyright 2015 The Rust Project Developers. See the COPYRIGHT
            // file at the top-level directory of this distribution and at
            // http://rust-lang.org/COPYRIGHT.
            //
            // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
            // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
            // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
            // option. This file may not be copied, modified, or distributed
            // except according to those terms.

            const K: u64 = 0x517cc1b727220a95u64;

            let mut hash = 0u64;
            let mut b = 0;

            while b + 8 <= bytes.len() {
                let i = [
                    bytes[b + 0],
                    bytes[b + 1],
                    bytes[b + 2],
                    bytes[b + 3],
                    bytes[b + 4],
                    bytes[b + 5],
                    bytes[b + 6],
                    bytes[b + 7],
                ];

                let i = u64::from_ne_bytes(i);

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);

                b += 8;
            }

            if b + 4 <= bytes.len() {
                let i = [bytes[b + 0], bytes[b + 1], bytes[b + 2], bytes[b + 3]];

                let i = u32::from_ne_bytes(i) as u64;

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);

                b += 4;
            }

            if b + 2 <= bytes.len() {
                let i = [bytes[b + 0], bytes[b + 1]];

                let i = u16::from_ne_bytes(i) as u64;

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);

                b += 2;
            }

            if b + 1 <= bytes.len() {
                let i = bytes[b + 0] as u64;

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);
            }

            hash
        }

        Tag {
            id: compute_id(data.as_bytes()),
            data,
        }
    }
}

impl<'a> fmt::Debug for Tag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

/**
The index of a value in its parent context.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(u32);

impl Index {
    pub const fn new(index: u32) -> Self {
        Index(index)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl Value for () {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.tagged_begin(Some(tags::RUST_UNIT), None, None)?;
        stream.null()?;
        stream.tagged_begin(Some(tags::RUST_UNIT), None, None)
    }

    fn is_dynamic(&self) -> bool {
        false
    }
}

impl Value for bool {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.bool(*self)
    }

    fn is_dynamic(&self) -> bool {
        false
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}
