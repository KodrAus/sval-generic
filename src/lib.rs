/*!
Structured, streaming values.

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or parsed from some text or binary format.

Imagine you have some text (a `str`).
You can serialize that text in `sval` through a kind of visitor called a _stream_:

```
# fn wrap<'sval, MyStream: sval::Stream<'sval>>(my_stream: impl FnOnce() -> MyStream) -> sval::Result {
// A stream is a type that receives structured data
let mut stream: MyStream = my_stream();

// Begin a text blob
// We know the blob will have 11 bytes
// In sval, text can be streamed in fragments rather than all upfront
stream.text_begin(Some(11))?;

// Send the first part of the text blob as a fragment
stream.text_fragment("Hello ")?;

// Send the rest of the text blob as another fragment
stream.text_fragment("World")?;

// End the text blob
stream.text_end()?;
# Ok(())
# }
```

Streams are the fundamental concept in `sval` that encodes its data model.
They're represented by the [`Stream`] trait.
Streams are:

- **flat**: there's no nesting or recursion in how complex values are streamed.
- **resumable**: there's no arbitrarily sized values that can't be streamed across multiple calls.
The driver of a stream can limit any internal buffering independently of the shape of the data.
- **maybe borrowed**: streams carry a `'sval` lifetime that can be used to stream borrowed data.
Borrowing is optional though so computed data can always flow through.

Streams are visitors that need to be driven by some data source.
If you're working with Rust `struct`s and `enum`s that data source can be a [`Value`].
Values are:

- **recursive**: complex values with fields containing other values will stream them through recursion.
- **repeatable**: values can be streamed multiple times.
- **borrowed**: values are always streamed through a `&'sval` reference.

Implementations of `Value` aren't the only source of data that can drive a stream.
They're just a reasonable driver for Rust objects that are provided by this crate.
The `Stream` trait makes very few demands on how it's fed data so could be used with buffering readers, async/await, or coroutines.
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
        core::{convert, fmt, mem, ops, result, str},
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
    value.stream(stream)
}
