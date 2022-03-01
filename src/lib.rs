/*!
Structured, streaming values

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or some text or binary format.

# Data model

`sval` uses a JSON-like data model:

- Values
    - Primitives
        - Unit (`()`)
        - Null
        - Unsigned integers (`u8` - `u128`)
        - Signed integers (`i8` - `i128`)
        - Floating points (`f32` - `f64`)
        - Booleans
    - Text
        - Unicode codepoints (`char`)
        - UTF8 strings (`str`)
        - Non-contiguous text fragments
    - Binary
        - Bytes (`&[u8]`)
        - Non-contiguous binary fragments
    - Maps (a set of heterogeneous value pairs)
    - Sequences (a set of heterogeneous values)

## Shape and tags

In Rust, every value has a _type_ that describes its properties:

```rust
let value: i32 = 42
# ;
```

In `sval`, every value also has a type, but this isn't necessarily the same as
its type in Rust. To avoid confusing the two, we say that every value in `sval` has
a _shape_ that describes its structure.

The shape of a value is everything a receiver may use to decide how to format it.
For simple types like booleans and integers this corresponds to the method on the `Receiver`
trait that accepts that value:

```rust
# fn with_receiver<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
receiver.i32(42)
# }
```
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
        alloc::{borrow, boxed, string, vec},
        core::{convert, fmt, mem, ops, result, str},
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
