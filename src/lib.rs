/*!
Structured, streaming values

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or some text or binary format.

Here's an example of how to stream a blob of text (a `str`) using `sval`:

```
# fn wrap<MyStream: sval::Stream<'a>>(my_stream: impl FnOnce() -> MyStream) -> sval::Result {
let mut stream: MyStream = my_stream();

stream.text_begin(Some(11))?;

stream.text_fragment("Hello ")?;
stream.text_fragment("World")?;

stream.text_end()?;
# Ok(())
# }
```

The [`Stream`] trait encodes `sval`'s data model.
Streams are:

- **flat**: there's no nesting or recursion in how complex values are streamed.
- **resumable**: there's no arbitrarily sized values that can't be streamed across multiple calls.
- **maybe borrowed**: streams carry a `'sval` lifetime that can be used to stream borrowed data.

The [`Value`] trait is a source that's suitable for Rust objects.
Values are:

- **recursive**: complex values with fields containing other values will stream them through recursion.
- **repeatable**: values can be streamed multiple times.
- **borrowed**: values are always streamed through a `&'sval` reference.
- **just one approach**: streams don't have to be driven by the `Value` trait.
The `Stream` trait makes very few demands so can be held in other ways, such as by async/await or coroutines.

`sval`'s data model encompasses more than what Rust can natively represent.
It's designed for cases where the consumer of structured data may be for a different language altogether so gives formats more tools for retaining the semantics of streamed data.
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
mod tag;
mod value;

pub mod result;

#[doc(inline)]
pub use self::{result::Error, stream::*, tag::*, value::*};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
