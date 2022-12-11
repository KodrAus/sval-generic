/*!
Structured, streaming values.

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or parsed from some encoding.

# Getting started

Add `sval` to your `Cargo.toml`:

```toml
[dependencies.sval]
version = "0.0.0"
```

By default, `sval` doesn't depend on Rust's standard library or integrate
with its collection types. To include them, add the `alloc` or `std` features:

```toml
[dependencies.sval]
version = "0.0.0"
features = ["std"]
```

## The `Value` trait

[`Value`] is a trait for data types to implement that surfaces their structure
through visitors called _streams_. `Value` is like `serde`'s `Serialize`.

## The `Stream` trait

[`Stream`] is a trait for data formats and visitors to implement that observes
the structure of _values_. `Stream` is like `serde`'s `Serializer`.
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
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, mem, ops, result, str},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

mod data;
mod stream;
mod value;

pub mod result;

#[doc(inline)]
pub use self::{data::*, result::Error, stream::*, value::*};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream<'sval, S: Stream<'sval> + ?Sized, V: Value + ?Sized>(
    stream: &mut S,
    value: &'sval V,
) -> Result {
    stream.value(value)
}
