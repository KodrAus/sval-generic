use crate::{data, Result, Source, Value};

/**
An observer of structured data emitted by some source.

# Data model

Receivers encode `sval`'s data model.

## Text and binary data

Each receiver expects either text-based or binary-based data.
This decision is communicated by [`Receiver::is_text_based`].
Some [data types](#data-types) may be streamed differently depending on whether a receiver is text-based or binary-based.

Receivers should only ever expect data encoded using either their text or binary representation.
This means `sval` effectively has two in-memory representations of its data model: one for text and one for binary.

## Data types

Data types represent the distinct kinds of data that a receiver may choose to interpret or encode in a particular way.

### Basic data types

The required methods on this trait represent the basic data model that all receivers need to understand.
The basic data model includes:

- **Unit**: the truthy value. See [`Receiver::unit`].
- **Null**: the falsey value. See [`Receiver::null`].
- **Text blobs**: UTF8 strings. See [`Receiver::text_begin`].
- **Binary blobs**: arbitrary byte strings. See [`Receiver::binary_begin`].
- **Maps**: homogenous collection of key-value pairs, where keys and values are [values](#values). See [`Receiver::map_begin`].
- **Sequences**: homogenous collection of values, where elements are [values](#values). See [`Receiver::seq_begin`].

All other data types map onto this basic model somehow.

### Extended data types

Receivers may opt-in to direct support for data types in the extended data model either as an optimization, or to handle them differently.
The extended data model includes:

- **Dynamic**: make [values](#values) heterogenous so that maps and sequences can contain values of different data types. See [`Receiver::dynamic_begin`].
- **Booleans**: the values `true` and `false`. See [`Receiver::bool`].
- **Integers**: `i8`-`i128`, `u8`-`u128` and arbitrarily sized. See [`Receiver::int_begin`] and [integer encoding](#integer-encoding).
- **Binary floating points**: `f32`-`f64` and arbitrarily sized. See [`Receiver::binfloat_begin`] and [binary floating point encoding](#binary-floating-point-encoding).
- **Decimal floating points**: These don't have a native Rust counterpart. See [`Receiver::decfloat_begin`] and [decimal floating point encoding](#decimal-floating-point-encoding).

## Values

A value is the sequence of calls that represent a complete [data type](#data-types).
The following are all examples of values.

A single integer:

```
# fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
receiver.i32(42)?;
# Ok(())
# }
```

A text blob, streamed as a contiguous borrowed value:

```
# fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
receiver.text("A blob of text")?;
# Ok(())
# }
```

A text blob, streamed as a collection of fragments:

```
# fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
receiver.text_begin(Some(14))?;

receiver.text_fragment("A blob ")?;
receiver.text_fragment("of text")?;

receiver.text_end()?;
# Ok(())
# }
```

A map of text-integer key-value pairs:

```
# fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
receiver.map_begin(Some(2))?;

receiver.map_key("a")?;
receiver.map_value(1)?;

receiver.map_key("b")?;
receiver.map_value(2)?;

receiver.map_end()?;
# Ok(())
# }
```

A receiver should expect just one value over its lifetime.

## Validation

Receivers aren't responsible for validating the correctness of the data they're given.
That's up to the caller to do.

## Forwarding

If a receiver is forwarding to another it should make an effort to forward all methods accurately, unless it's specifically transforming the data in some way.

# Borrowing

Receivers may accept text and binary data that's borrowed for a particular lifetime (`'data`).
Borrowing is just an optimization though, and receivers also need to expect data that's short-lived.

# Recursion and nesting

Some methods on a receiver accept a source as a parameter that needs to be streamed then and there ([`Receiver::map_key`] for example).
Methods that accept sources are just an optimization though, and receivers need to expect values will also be broken up into individual calls ([`Receiver::map_key_begin`] + [`Receiver::map_key_end`] for example).

Receivers need to manage the state they need across calls themselves, rather than relying on the callstack to hold it.
*/
pub trait Receiver<'data> {
    /**
    Whether or not the receiver expects text or binary data.

    This choice is expected to be constant over a single complete value.
    Callers are expected to check this method before choosing between the text or binary encoding for a particular [data type](#data-type).
    */
    fn is_text_based(&self) -> bool {
        true
    }

    /**
    A borrowed value.

    This is a niche method that simply calls back into the receiver, so shouldn't be called from [`Value::stream`].
    It can be useful for separating borrowed data out to avoid needing to buffer it.
    */
    fn value<V: Value + ?Sized + 'data>(&mut self, value: &'data V) -> Result {
        value.stream(self)
    }

    /**
    A value that simply _is_.

    Unit is one of the [basic data types](basic-data-types), but isn't commonly used directly.

    # Examples

    Stream a unit:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.unit()?;
    # Ok(())
    # }
    ```

    Rust's `()` type also streams as unit:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    ().stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    Unit is a distinct data type that only matches other units.
    That means unit and null are not the same data type, and unit and other values like `i32` are not the same data type.
    */
    fn unit(&mut self) -> Result;

    /**
    A value that simply _isn't_.

    Null is one of the [basic data types](basic-data-types), but isn't commonly used directly.
    Rust typically represents null through the `Option` type, which may also be the `Some` variant of another type.

    # Examples

    Stream a null:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.null()?;
    # Ok(())
    # }
    ```

    # Data type

    Null is a distinct data type that only matches other nulls.
    That means unit and null are not the same data type.

    Rust doesn't have a primitive type that maps to null.
    The `Option` type will stream its `None` variant as null, but wrapped in a nullable (see [`Receiver::nullable_begin`]) so that it
    has the same data type as its `Some` variant.
    That means that `Option::None` and null don't actually have the same data type.
    */
    fn null(&mut self) -> Result;

    /**
    The values `true` or `false`.

    Boolean is one of the [extended data types](extended-data-types).

    # Examples

    Stream a boolean:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.bool(true)?;
    # Ok(())
    # }
    ```

    Rust's `bool` type also streams as a boolean:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    true.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    Boolean is a distinct data type that only matches other booleans.
    The values `true` and `false` do have the same data type.

    # Boolean encoding

    Booleans map to the basic data model as an empty nullable, so `true` will become unit (see [`Receiver::unit`]) and `false` will become null (see [`Receiver::null`]).
    Also see [`Receiver::nullable_begin`] for more details.
    */
    fn bool(&mut self, value: bool) -> Result {
        // This streams as a nullable (Option<()>)
        value.then(|| ()).stream_to_end(self)
    }

    /**
    Begin a UTF8 text blob.

    Text blobs are one of the [basic data types](basic-data-types).
    Most other data types map to text blobs for [text-based receivers](text-and-binary-data), but binary-based receivers may also stream text.

    The `num_bytes_hint` argument is a hint for how many bytes the text blob will contain.
    If a hint is given it should be as accurate as possible.

    Also see [`Receiver::text`] as a simpler alternative that streams a borrowed string as a text blob.

    # Structure

    After beginning a text blob, the receiver should only expect zero or more text fragments ([`Receiver::text_fragment`] or [`Receiver::text_fragment_computed`]) followed by a call to [`Receiver::text_end`]:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.text_begin(num_bytes_hint)?;

    // 0 or more calls to any combination of text_fragment and text_fragment_computed

    receiver.text_end()?;
    # Ok(())
    # }
    ```

    # Borrowing

    Text blobs may contain data that's borrowed for the receiver's `'data` lifetime.
    Fragments streamed using [`Receiver::text_fragment`] will be borrowed for `'data`.
    Fragments streamed using [`Receiver::text_fragment_computed`] will be arbitrarily short-lived.

    Callers should use data borrowed for `'data` wherever possible.
    Borrowing is just an optimization though, so receivers need to cater to both cases.

    # Examples

    Stream a text blob using a single string:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.text_begin(Some(14))?;

    receiver.text_fragment("A blob of text")?;

    receiver.text_end()?;
    # Ok(())
    # }
    ```

    Rust's `str` type also streams as a text blob:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    "A blob of text".stream(receiver)?;
    # Ok(())
    # }
    ```

    Types that implement the standard `Display` trait can be streamed using the [`data::display`] utility:

    ```
    # use sval::Value;
    # fn wrap<R: for<'a> sval::Receiver<'a>>(mut receiver: R) -> sval::Result {
    sval::data::display(42).stream(receiver)?;
    # Ok(())
    # }
    ```

    Text may need to be computed instead of just being available.
    The [`Receiver::text_fragment_computed`] method can be used to stream text that doesn't satisfy the `'data` lifetime:

    ```
    # fn compute_text() -> String { Default::default() }
    # fn wrap<'a>(borrowed_text: &'a str, mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.text_begin(None)?;

    // This borrowed text lives for `'data`
    receiver.text_fragment(borrowed_text)?;

    // This owned text only lives until the end of our function call
    // So we need to stream it as a computed fragment
    let s: String = compute_text();
    receiver.text_fragment_computed(&s)?;

    receiver.text_end()?;
    # Ok(())
    # }
    ```
    */
    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    /**
    A UTF8 text fragment that's borrowed for `'data`.

    See [`Receiver::text_begin`] for details on text fragments.
    The [`Receiver::text_fragment_computed`] method is an alternative to this one that doesn't need to borrow for `'data`.
    */
    fn text_fragment(&mut self, fragment: &'data str) -> Result {
        self.text_fragment_computed(fragment)
    }

    /**
    A UTF8 text fragment that's borrowed for some arbitrarily short lifetime.

    See [`Receiver::text_begin`] for details on text fragments.
    The [`Receiver::text_fragment`] method is an alternative to this one that borrows for `'data`.
    */
    fn text_fragment_computed(&mut self, fragment: &str) -> Result;

    /**
    End a UTF8 text blob.

    See [`Receiver::text_begin`] for details on text fragments.
    */
    fn text_end(&mut self) -> Result;

    /**
    Stream a text blob as a single, contiguous fragment borrowed for `'data`.

    See [`Receiver::text_begin`] for details on text fragments.

    # Examples

    Stream a text blob using a single string:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.text("A blob of text")?;
    # Ok(())
    # }
    ```

    Rust's `str` type also streams as a single contiguous text blob:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    "A blob of text".stream(receiver)?;
    # Ok(())
    # }
    ```
    */
    fn text(&mut self, value: &'data str) -> Result {
        self.text_begin(Some(value.len()))?;
        self.text_fragment(value)?;
        self.text_end()
    }

    /**
    Begin a binary blob.

    Binary blobs are one of the [basic data types](basic-data-types).
    Most other data types map to binary blobs for [binary-based receivers](text-and-binary-data), but text-based receivers may also stream binary.

    The `num_bytes_hint` argument is a hint for how many bytes the binary blob will contain.
    If a hint is given it should be as accurate as possible.

    Also see [`Receiver::binary`] as a simpler alternative that streams a borrowed slice as a binary blob.

    # Structure

    After beginning a binary blob, the receiver should only expect zero or more binary fragments ([`Receiver::binary_fragment`] or [`Receiver::binary_fragment_computed`]) followed by a call to [`Receiver::binary_end`]:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.binary_begin(num_bytes_hint)?;

    // 0 or more calls to any combination of binary_fragment and binary_fragment_computed

    receiver.binary_end()?;
    # Ok(())
    # }
    ```

    # Borrowing

    Binary blobs may contain data that's borrowed for the receiver's `'data` lifetime.
    Fragments streamed using [`Receiver::binary_fragment`] will be borrowed for `'data`.
    Fragments streamed using [`Receiver::binary_fragment_computed`] will be arbitrarily short-lived.

    Callers should use data borrowed for `'data` wherever possible.
    Borrowing is just an optimization though, so receivers need to cater to both cases.

    # Examples

    Stream a binary blob using a single string:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.binary_begin(Some(5))?;

    receiver.binary_fragment(&[0xaa, 0xbb, 0xcc, 0xdd, 0x00])?;

    receiver.binary_end()?;
    # Ok(())
    # }
    ```

    Slices of bytes (`[u8]`) aren't directly streamed as binary, but the [`data::binary`] utility will wrap one so that it will:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    sval::data::binary(&[0xaa, 0xbb, 0xcc, 0xdd, 0x00]).stream(receiver)?;
    # Ok(())
    # }
    ```

    Binary may need to be computed instead of just being available.
    The [`Receiver::binary_fragment_computed`] method can be used to stream binary that doesn't satisfy the `'data` lifetime:

    ```
    # fn compute_binary() -> Vec<u8> { Default::default() }
    # fn wrap<'a>(borrowed_binary: &'a [u8], mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.binary_begin(None)?;

    // This borrowed binary lives for `'data`
    receiver.binary_fragment(borrowed_binary)?;

    // This owned binary only lives until the end of our function call
    // So we need to stream it as a computed fragment
    let s: Vec<u8> = compute_binary();
    receiver.binary_fragment_computed(&s)?;

    receiver.binary_end()?;
    # Ok(())
    # }
    ```
    */
    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    /**
    A binary fragment that's borrowed for `'data`.

    See [`Receiver::binary_begin`] for details on binary fragments.
    The [`Receiver::binary_fragment_computed`] method is an alternative to this one that doesn't need to borrow for `'data`.
    */
    fn binary_fragment(&mut self, fragment: &'data [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

    /**
    A binary fragment that's borrowed for some arbitrarily short lifetime.

    See [`Receiver::binary_begin`] for details on binary fragments.
    The [`Receiver::binary_fragment`] method is an alternative to this one that borrows for `'data`.
    */
    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result;

    /**
    End a binary blob.

    See [`Receiver::binary_begin`] for details on binary fragments.
    */
    fn binary_end(&mut self) -> Result;

    /**
    Stream a binary blob as a single, contiguous fragment borrowed for `'data`.

    See [`Receiver::binary_begin`] for details on binary fragments.

    # Examples

    Stream a binary blob using a single string:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.binary(&[0xaa, 0xbb, 0xcc, 0xdd, 0x00])?;
    # Ok(())
    # }
    ```

    Slices of bytes (`[u8]`) aren't directly streamed as binary, but the [`data::binary`] utility will wrap one so that it will:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    sval::data::binary(&[0xaa, 0xbb, 0xcc, 0xdd, 0x00]).stream(receiver)?;
    # Ok(())
    # }
    ```
    */
    fn binary(&mut self, value: &'data [u8]) -> Result {
        self.binary_begin(Some(value.len()))?;
        self.binary_fragment(value)?;
        self.binary_end()
    }

    /**
    Stream an 8bit unsigned integer.

    `u8`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u8`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.u8(42)?;
    # Ok(())
    # }
    ```

    Rust's `u8` type also streams as an 8bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42u8.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `u8` is a distinct data type that only matches other `u8`s.
    That means `u8` doesn't have the same type as `i8`, `u16`, or arbitrary sized integers.

    # `u8` encoding

    `u8`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn u8(&mut self, value: u8) -> Result {
        data::u8_int(value, self)
    }

    /**
    Stream a 16bit unsigned integer.

    `u16`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u16`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.u16(42)?;
    # Ok(())
    # }
    ```

    Rust's `u16` type also streams as a 16bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42u16.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `u16` is a distinct data type that only matches other `u16`s.
    That means `u16` doesn't have the same type as `i16`, `u8`, or arbitrary sized integers.

    # `u16` encoding

    `u16`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn u16(&mut self, value: u16) -> Result {
        data::u16_int(value, self)
    }

    /**
    Stream a 32bit unsigned integer.

    `u32`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u32`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.u32(42)?;
    # Ok(())
    # }
    ```

    Rust's `u32` type also streams as a 32bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42u32.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `u32` is a distinct data type that only matches other `u32`s.
    That means `u32` doesn't have the same type as `i32`, `f32`, `u64`, or arbitrary sized integers.

    # `u32` encoding

    `u32`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn u32(&mut self, value: u32) -> Result {
        data::u32_int(value, self)
    }

    /**
    Stream a 64bit unsigned integer.

    `u64`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u64`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.u64(42)?;
    # Ok(())
    # }
    ```

    Rust's `u64` type also streams as a 64bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42u64.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `u64` is a distinct data type that only matches other `u64`s.
    That means `u64` doesn't have the same type as `i64`, `f64`, `u128`, or arbitrary sized integers.

    # `u64` encoding

    `u64`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn u64(&mut self, value: u64) -> Result {
        data::u64_int(value, self)
    }

    /**
    Stream a 128bit unsigned integer.

    `u128`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u128`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.u128(42)?;
    # Ok(())
    # }
    ```

    Rust's `u128` type also streams as a 128bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42u128.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `u128` is a distinct data type that only matches other `u128`s.
    That means `u128` doesn't have the same type as `i128` or arbitrary sized integers.

    # `u128` encoding

    `u128`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn u128(&mut self, value: u128) -> Result {
        data::u128_int(value, self)
    }

    /**
    Stream an 8bit signed integer.

    `i8`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i8`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.i8(42)?;
    # Ok(())
    # }
    ```

    Rust's `i8` type also streams as an 8bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42i8.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `i8` is a distinct data type that only matches other `i8`s.
    That means `i8` doesn't have the same type as `u8`, `i16`, or arbitrary sized integers.

    # `i8` encoding

    `i8`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn i8(&mut self, value: i8) -> Result {
        data::i8_int(value, self)
    }

    /**
    Stream a 16bit signed integer.

    `i16`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i16`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.i16(42)?;
    # Ok(())
    # }
    ```

    Rust's `i16` type also streams as a 16bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42i16.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `i16` is a distinct data type that only matches other `i16`s.
    That means `i16` doesn't have the same type as `u16`, `i8`, or arbitrary sized integers.

    # `i16` encoding

    `i16`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn i16(&mut self, value: i16) -> Result {
        data::i16_int(value, self)
    }

    /**
    Stream a 32bit signed integer.

    `i32`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i32`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.i32(42)?;
    # Ok(())
    # }
    ```

    Rust's `i32` type also streams as a 32bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42i32.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `i32` is a distinct data type that only matches other `i32`s.
    That means `i32` doesn't have the same type as `u32`, `f32`, `i64`, or arbitrary sized integers.

    # `i32` encoding

    `i32`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn i32(&mut self, value: i32) -> Result {
        data::i32_int(value, self)
    }

    /**
    Stream a 64bit signed integer.

    `i64`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i64`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.i64(42)?;
    # Ok(())
    # }
    ```

    Rust's `i64` type also streams as a 64bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42i64.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `i64` is a distinct data type that only matches other `i64`s.
    That means `i64` doesn't have the same type as `u64`, `f64`, `i128`, or arbitrary sized integers.

    # `i64` encoding

    `i64`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn i64(&mut self, value: i64) -> Result {
        data::i64_int(value, self)
    }

    /**
    Stream a 128bit signed integer.

    `i128`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i128`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.i128(42)?;
    # Ok(())
    # }
    ```

    Rust's `i128` type also streams as a 128bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    42i128.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `i128` is a distinct data type that only matches other `i128`s.
    That means `i128` doesn't have the same type as `u128` or arbitrary sized integers.

    # `i128` encoding

    `i128`s map to the basic data model as a text or binary blob containing an integer.
    See [`Receiver::int_begin`] for more details.
    */
    fn i128(&mut self, value: i128) -> Result {
        data::i128_int(value, self)
    }

    /**
    Stream a 32bit binary floating number.

    `f32`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `f32`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.f32(4.2)?;
    # Ok(())
    # }
    ```

    Rust's `f32` type also streams as a 32bit binary floating number:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    4.2f32.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `f32` is a distinct data type that only matches other `f32`s.
    That means `f32` doesn't have the same type as `i32`, `f64`, or arbitrary sized floating points.

    # `f32` encoding

    `f32`s map to the basic data model as a text or binary blob containing a binary floating point number.
    See [`Receiver::binfloat_begin`] for more details.
    */
    fn f32(&mut self, value: f32) -> Result {
        data::f32_number(value, self)
    }

    /**
    Stream a 64bit binary floating number.

    `f64`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `f64`:

    ```
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.f64(4.2)?;
    # Ok(())
    # }
    ```

    Rust's `f64` type also streams as a 64bit binary floating number:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    4.2f64.stream(receiver)?;
    # Ok(())
    # }
    ```

    # Data type

    `f64` is a distinct data type that only matches other `f64`s.
    That means `f64` doesn't have the same type as `f32`, or arbitrary sized floating points.

    # `f64` encoding

    `f64`s map to the basic data model as a text or binary blob containing a binary floating point number.
    See [`Receiver::binfloat_begin`] for more details.
    */
    fn f64(&mut self, value: f64) -> Result {
        data::f64_number(value, self)
    }

    /**
    Begin a homogenous map of key-value pairs.

    The [data type](data-types) of all keys and the [data type](data-types) of all values must be the same.

    # Structure

    Maps must contain zero or more pairs of keys and values, followed by a call to [`Receiver::map_end`].
    */
    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    fn map_key_begin(&mut self) -> Result;

    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;

    fn map_value_end(&mut self) -> Result;

    fn map_end(&mut self) -> Result;

    fn map_key<'k: 'data, K: Source<'k>>(&mut self, mut key: K) -> Result {
        self.map_key_begin()?;
        key.stream_to_end(&mut *self)?;
        self.map_key_end()
    }

    fn map_value<'v: 'data, V: Source<'v>>(&mut self, mut value: V) -> Result {
        self.map_value_begin()?;
        value.stream_to_end(&mut *self)?;
        self.map_value_end()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    fn seq_value_begin(&mut self) -> Result;

    fn seq_value_end(&mut self) -> Result;

    fn seq_end(&mut self) -> Result;

    fn seq_value<'e: 'data, V: Source<'e>>(&mut self, mut value: V) -> Result {
        self.seq_value_begin()?;
        value.stream_to_end(&mut *self)?;
        self.seq_value_end()
    }

    fn dynamic_begin(&mut self) -> Result {
        Ok(())
    }

    fn dynamic_end(&mut self) -> Result {
        Ok(())
    }

    fn fixed_size_begin(&mut self) -> Result {
        Ok(())
    }

    fn fixed_size_end(&mut self) -> Result {
        Ok(())
    }

    fn tagged_begin(&mut self, tag: data::Tag) -> Result {
        let _ = tag;

        Ok(())
    }

    fn tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn constant_begin(&mut self, tag: data::Tag) -> Result {
        self.tagged_begin(tag)
    }

    fn constant_end(&mut self) -> Result {
        self.tagged_end()
    }

    fn struct_map_begin(&mut self, tag: data::Tag, num_entries_hint: Option<usize>) -> Result {
        self.tagged_begin(tag)?;
        self.map_begin(num_entries_hint)
    }

    fn struct_map_key_begin(&mut self, tag: data::Tag) -> Result {
        let _ = tag;

        self.map_key_begin()?;
        self.dynamic_begin()
    }

    fn struct_map_key_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.map_key_end()
    }

    fn struct_map_value_begin(&mut self, tag: data::Tag) -> Result {
        let _ = tag;

        self.map_value_begin()?;
        self.dynamic_begin()
    }

    fn struct_map_value_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.map_value_end()
    }

    fn struct_map_end(&mut self) -> Result {
        self.map_end()
    }

    fn struct_map_key<'k: 'data, K: Source<'k>>(&mut self, tag: data::Tag, mut key: K) -> Result {
        self.struct_map_key_begin(tag)?;
        key.stream_to_end(&mut *self)?;
        self.struct_map_key_end()
    }

    fn struct_map_value<'v: 'data, V: Source<'v>>(
        &mut self,
        tag: data::Tag,
        mut value: V,
    ) -> Result {
        self.struct_map_value_begin(tag)?;
        value.stream_to_end(&mut *self)?;
        self.struct_map_value_end()
    }

    fn struct_seq_begin(&mut self, tag: data::Tag, num_entries_hint: Option<usize>) -> Result {
        self.tagged_begin(tag)?;
        self.seq_begin(num_entries_hint)
    }

    fn struct_seq_value_begin(&mut self, tag: data::Tag) -> Result {
        let _ = tag;

        self.seq_value_begin()?;
        self.dynamic_begin()
    }

    fn struct_seq_value_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.seq_value_end()
    }

    fn struct_seq_end(&mut self) -> Result {
        self.seq_end()
    }

    fn struct_seq_value<'v: 'data, V: Source<'v>>(
        &mut self,
        tag: data::Tag,
        mut value: V,
    ) -> Result {
        self.struct_seq_value_begin(tag)?;
        value.stream_to_end(&mut *self)?;
        self.struct_seq_value_end()
    }

    fn enum_begin(&mut self, tag: data::Tag) -> Result {
        self.tagged_begin(tag)?;
        self.dynamic_begin()
    }

    fn enum_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.tagged_end()
    }

    fn nullable_begin(&mut self) -> Result {
        self.dynamic_begin()
    }

    fn nullable_end(&mut self) -> Result {
        self.dynamic_end()
    }

    /**
    Begin an arbitrarily sized integer.

    # Structure

    Arbitrary sized integers wrap a text or binary blob with the encoding described below.
    A call to `int_begin` must be followed by a call to `int_end` after the integer value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.int_begin()?;

    if receiver.is_text_based() {
        receiver.text("754")?;
    } else {
        receiver.binary(&[0b11110010, 0b00000010])?;
    }

    receiver.int_end()?;
    # Ok(())
    # }
    ```

    # Integer encoding

    Each kind of integer is considered a different data type.
    So `u8` is a different type to `i8` and `u8` is a different type to `u16`.
    All arbitarily sized integers (those streamed using [`Receiver::int_begin`]) are considered the same type.

    `i8`-`i128`, `u8`-`u128`, and arbitrary-sized integers use the same text-based or binary-based encoding described below.

    For [text-based receivers](#text-and-binary-data), integers map to text blobs representing a base10 number with the following grammar:

    ```text
    -?[0-9]+
    ```

    For [binary-based receivers](#binary-based-receivers), integers map to signed, little-endian, two's-compliment bytes.

    The following table shows some example integers along with their text and binary encodings.
    The binary encoding uses the smallest possible representation, even though that's not a requirement.

    | Integer | Text encoding | Binary encoding     |
    | ------- | ------------: | ------------------: |
    | 0       | `0`           | `00000000`          |
    | 754     | `754`         | `11110010_00000010` |
    | -754    | `-754`        | `00001110_11111101` |
    */
    fn int_begin(&mut self) -> Result {
        Ok(())
    }

    /**
    End an arbitrary sized integer.

    See [`Receiver::int_begin`] for details on arbitrary sized integers.
    */
    fn int_end(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin an arbitrarily sized binary floating point number.

    # Structure

    Arbitrary sized binary floating points wrap a text or binary blob with the encoding described below.
    A call to `binfloat_begin` must be followed by a call to `binfloat_end` after the floating point value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.binfloat_begin()?;

    if receiver.is_text_based() {
        receiver.text("1333.754")?;
    } else {
        receiver.binary(&[0b00100001, 0b10111000, 0b10100110, 0b01000100])?;
    }

    receiver.binfloat_end()?;
    # Ok(())
    # }
    ```

    # Binary floating point encoding

    `f32` is a different type to `f64`.
    All arbitrarily sized binary floating points (those streamed using [`Receiver::binfloat_begin`]) are considered the same type, regardless of size.

    `f32`, `f64`, and arbitrarily-sized floating points use the same text-based or binary-based encoding described below.

    For [text-based receivers](#text-and-binary-data), binary floating points map to text blobs representing a base10 number with the following case-insensitive grammar:

    ```text
    inf|[-+]?(nan|[0-9]+(\.[0-9]+)?)
    ```

    For [binary-based receivers](#text-and-binary-data), binary floating points map to little-endian IEEE754 interchange binary floating points.

    The following table shows some example binary floating points along with their text and binary encodings.
    The binary encoding uses the smallest possible representation, even though that's not a requirement.

    | Number            | Text encoding | Binary encoding                       |
    | ----------------- | ------------: | ------------------------------------: |
    | NaN               | `nan`         | `00000000_01111110`                   |
    | Positive infinity | `inf`         | `00000000_01111100`                   |
    | Negative infinity | `-inf`        | `00000000_11111100`                   |
    | 1333.754          | `1333.754`    | `00100001_10111000_10100110_01000100` |
    | -1333.754         | `-1333.754`   | `00100001_10111000_10100110_11000100` |
    | 0                 | `0`           | `00000000_00000000`                   |
    | -0                | `-0`          | `00000000_10000000`                   |
    */
    fn binfloat_begin(&mut self) -> Result {
        Ok(())
    }

    /**
    End an arbitrary sized binary floating point number.

    See [`Receiver::binfloat_begin`] for details on arbitrary sized binary floating points.
    */
    fn binfloat_end(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin an arbitrarily sized decimal floating point number.

    # Structure

    Arbitrary sized decimal floating points wrap a text or binary blob with the encoding described below.
    A call to `decfloat_begin` must be followed by a call to `decfloat_end` after the floating point value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut receiver: impl sval::Receiver<'a>) -> sval::Result {
    receiver.decfloat_begin()?;

    if receiver.is_text_based() {
        receiver.text("1333.754")?;
    } else {
        receiver.binary(&[0b1101010, 0b1100111, 0b0010011, 0b00100110])?;
    }

    receiver.decfloat_end()?;
    # Ok(())
    # }
    ```

    # Decimal floating point encoding

    Rust doesn't have any native decimal floating point types.
    All arbitrarily sized decimal floating points (those streamed using [`Receiver::decfloat_begin`]) are considered the same type.

    For [text-based receivers](#text-and-binary-data), decimal floating points use the same encoding as [binary floating points](#binary-floating-point-encoding).

    For [binary-based receivers](#text-and-binary-data), decimal floating points map to little-endian IEEE754 interchange decimal floating points using the [densely-packed-decimal](https://en.wikipedia.org/wiki/Densely_packed_decimal) representation.

    | Number            | Text encoding | Binary encoding                       |
    | ----------------- | ------------: | ------------------------------------: |
    | NaN               | `nan`         | `00000000_00000000_00000000_01111100` |
    | Positive infinity | `inf`         | `00000000_00000000_00000000_01111000` |
    | Negative infinity | `-inf`        | `00000000_00000000_00000000_11111000` |
    | 1333.754          | `1333.754`    | `11010100_11001111_00100110_00100110` |
    | -1333.754         | `-1333.754`   | `11010100_11001111_00100110_10100110` |
    | 0                 | `0`           | `00000000_00000000_01010000_00100010` |
    | -0                | `-0`          | `00000000_00000000_01010000_10100010` |
    */
    fn decfloat_begin(&mut self) -> Result {
        Ok(())
    }

    /**
    End an arbitrary sized decimal floating point number.

    See [`Receiver::decfloat_begin`] for details on arbitrary sized decimal floating points.
     */
    fn decfloat_end(&mut self) -> Result {
        Ok(())
    }
}

macro_rules! impl_receiver_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn is_text_based(&self) -> bool {
                let $bind = self;
                ($($forward)*).is_text_based()
            }

            fn value<V: Value + ?Sized + 'data>(&mut self, value: &'data V) -> Result {
                let $bind = self;
                ($($forward)*).value(value)
            }

            fn dynamic_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).dynamic_begin()
            }

            fn dynamic_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).dynamic_end()
            }

            fn unit(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).unit()
            }

            fn null(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).null()
            }

            fn u8(&mut self, value: u8) -> Result {
                let $bind = self;
                ($($forward)*).u8(value)
            }

            fn u16(&mut self, value: u16) -> Result {
                let $bind = self;
                ($($forward)*).u16(value)
            }

            fn u32(&mut self, value: u32) -> Result {
                let $bind = self;
                ($($forward)*).u32(value)
            }

            fn u64(&mut self, value: u64) -> Result {
                let $bind = self;
                ($($forward)*).u64(value)
            }

            fn u128(&mut self, value: u128) -> Result {
                let $bind = self;
                ($($forward)*).u128(value)
            }

            fn i8(&mut self, value: i8) -> Result {
                let $bind = self;
                ($($forward)*).i8(value)
            }

            fn i16(&mut self, value: i16) -> Result {
                let $bind = self;
                ($($forward)*).i16(value)
            }

            fn i32(&mut self, value: i32) -> Result {
                let $bind = self;
                ($($forward)*).i32(value)
            }

            fn i64(&mut self, value: i64) -> Result {
                let $bind = self;
                ($($forward)*).i64(value)
            }

            fn i128(&mut self, value: i128) -> Result {
                let $bind = self;
                ($($forward)*).i128(value)
            }

            fn f32(&mut self, value: f32) -> Result {
                let $bind = self;
                ($($forward)*).f32(value)
            }

            fn f64(&mut self, value: f64) -> Result {
                let $bind = self;
                ($($forward)*).f64(value)
            }

            fn bool(&mut self, value: bool) -> Result {
                let $bind = self;
                ($($forward)*).bool(value)
            }

            fn text(&mut self, value: &'data str) -> Result {
                let $bind = self;
                ($($forward)*).text(value)
            }

            fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).text_begin(num_bytes_hint)
            }

            fn text_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).text_end()
            }

            fn text_fragment(&mut self, fragment: &'data str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment(fragment)
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment_computed(fragment)
            }

            fn binary(&mut self, value: &'data [u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary(value)
            }

            fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).binary_begin(num_bytes_hint)
            }

            fn binary_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).binary_end()
            }

            fn binary_fragment(&mut self, fragment: &'data [u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary_fragment(fragment)
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary_fragment_computed(fragment)
            }

            fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).map_begin(num_entries_hint)
            }

            fn map_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_end()
            }

            fn map_key_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_key_begin()
            }

            fn map_key_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_key_end()
            }

            fn map_value_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_value_begin()
            }

            fn map_value_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_value_end()
            }

            fn map_key<'k: 'data, K: Source<'k>>(&mut self, key: K) -> Result {
                let $bind = self;
                ($($forward)*).map_key(key)
            }

            fn map_value<'v: 'data, V: Source<'v>>(&mut self, value: V) -> Result {
                let $bind = self;
                ($($forward)*).map_value(value)
            }

            fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).seq_begin(num_entries_hint)
            }

            fn seq_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_end()
            }

            fn seq_value_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_value_begin()
            }

            fn seq_value_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_value_end()
            }

            fn seq_value<'e: 'data, V: Source<'e>>(&mut self, value: V) -> Result {
                let $bind = self;
                ($($forward)*).seq_value(value)
            }

            fn tagged_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag)
            }

            fn tagged_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).tagged_end()
            }

            fn constant_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).constant_begin(tag)
            }

            fn constant_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).constant_end()
            }

            fn struct_map_begin(&mut self, tag: data::Tag, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_begin(tag, num_entries_hint)
            }

            fn struct_map_key_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_key_begin(tag)
            }

            fn struct_map_key_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_key_end()
            }

            fn struct_map_value_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_value_begin(tag)
            }

            fn struct_map_value_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_value_end()
            }

            fn struct_map_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_end()
            }

            fn struct_map_key<'k: 'data, K: Source<'k>>(&mut self, tag: data::Tag, key: K) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_key(tag, key)
            }

            fn struct_map_value<'v: 'data, V: Source<'v>>(&mut self, tag: data::Tag, value: V) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_value(tag, value)
            }

            fn struct_seq_begin(&mut self, tag: data::Tag, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).struct_seq_begin(tag, num_entries_hint)
            }

            fn struct_seq_value_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).struct_seq_value_begin(tag)
            }

            fn struct_seq_value_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).struct_seq_value_end()
            }

            fn struct_seq_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).struct_seq_end()
            }

            fn struct_seq_value<'v: 'data, V: Source<'v>>(&mut self, tag: data::Tag, value: V) -> Result {
                let $bind = self;
                ($($forward)*).struct_seq_value(tag, value)
            }

            fn enum_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).enum_begin(tag)
            }

            fn enum_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).enum_end()
            }

            fn nullable_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).nullable_begin()
            }

            fn nullable_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).nullable_end()
            }

            fn fixed_size_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).fixed_size_begin()
            }

            fn fixed_size_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).fixed_size_end()
            }

            fn int_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).int_begin()
            }

            fn int_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).int_end()
            }

            fn binfloat_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).binfloat_begin()
            }

            fn binfloat_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).binfloat_end()
            }

            fn decfloat_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).decfloat_begin()
            }

            fn decfloat_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).decfloat_end()
            }
        }
    };
}

// Simplifies the default receivers for extracting concrete types from values
pub(crate) trait DefaultUnsupported<'data> {
    fn as_receiver(&mut self) -> AsReceiver<&mut Self> {
        AsReceiver(self)
    }

    fn is_text_based(&self) -> bool {
        false
    }

    fn value<V: Value + ?Sized + 'data>(&mut self, v: &'data V) -> Result {
        v.stream(self.as_receiver())
    }

    fn dynamic_begin(&mut self) -> Result {
        Ok(())
    }

    fn dynamic_end(&mut self) -> Result {
        Ok(())
    }

    fn unit(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn null(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn u8(&mut self, _: u8) -> Result {
        crate::error::unsupported()
    }

    fn u16(&mut self, _: u16) -> Result {
        crate::error::unsupported()
    }

    fn u32(&mut self, _: u32) -> Result {
        crate::error::unsupported()
    }

    fn u64(&mut self, _: u64) -> Result {
        crate::error::unsupported()
    }

    fn u128(&mut self, _: u128) -> Result {
        crate::error::unsupported()
    }

    fn i8(&mut self, _: i8) -> Result {
        crate::error::unsupported()
    }

    fn i16(&mut self, _: i16) -> Result {
        crate::error::unsupported()
    }

    fn i32(&mut self, _: i32) -> Result {
        crate::error::unsupported()
    }

    fn i64(&mut self, _: i64) -> Result {
        crate::error::unsupported()
    }

    fn i128(&mut self, _: i128) -> Result {
        crate::error::unsupported()
    }

    fn f32(&mut self, _: f32) -> Result {
        crate::error::unsupported()
    }

    fn f64(&mut self, _: f64) -> Result {
        crate::error::unsupported()
    }

    fn bool(&mut self, _: bool) -> Result {
        crate::error::unsupported()
    }

    fn text(&mut self, _: &'data str) -> Result {
        crate::error::unsupported()
    }

    fn text_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn text_fragment(&mut self, _: &'data str) -> Result {
        crate::error::unsupported()
    }

    fn text_fragment_computed(&mut self, _: &str) -> Result {
        crate::error::unsupported()
    }

    fn text_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn binary(&mut self, _: &'data [u8]) -> Result {
        crate::error::unsupported()
    }

    fn binary_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn binary_fragment(&mut self, _: &'data [u8]) -> Result {
        crate::error::unsupported()
    }

    fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
        crate::error::unsupported()
    }

    fn binary_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn map_key_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_key_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_value_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_value_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_key<'k: 'data, K: Source<'k>>(&mut self, _: K) -> Result {
        crate::error::unsupported()
    }

    fn map_value<'v: 'data, V: Source<'v>>(&mut self, _: V) -> Result {
        crate::error::unsupported()
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn seq_value_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn seq_value_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn seq_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn seq_value<'e: 'data, E: Source<'e>>(&mut self, _: E) -> Result {
        crate::error::unsupported()
    }

    fn tagged_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn tagged_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn constant_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn constant_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_begin(&mut self, _: data::Tag, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_key_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_key_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_value_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_value_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_key<'k: 'data, K: Source<'k>>(&mut self, _: data::Tag, _: K) -> Result {
        crate::error::unsupported()
    }

    fn struct_map_value<'v: 'data, V: Source<'v>>(&mut self, _: data::Tag, _: V) -> Result {
        crate::error::unsupported()
    }

    fn struct_seq_begin(&mut self, _: data::Tag, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn struct_seq_value_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn struct_seq_value_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn struct_seq_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn struct_seq_value<'v: 'data, V: Source<'v>>(&mut self, _: data::Tag, _: V) -> Result {
        crate::error::unsupported()
    }

    fn enum_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn enum_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn nullable_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn nullable_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn fixed_size_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn fixed_size_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn int_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn int_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn binfloat_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn binfloat_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn decfloat_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn decfloat_end(&mut self) -> Result {
        crate::error::unsupported()
    }
}

pub(crate) struct AsReceiver<T: ?Sized>(T);

impl_receiver_forward!({ impl<'data, 'a, R: ?Sized> Receiver<'data> for &'a mut R where R: Receiver<'data> } => x => { **x });
impl_receiver_forward!({ impl<'data, 'a, R: ?Sized> Receiver<'data> for AsReceiver<&'a mut R> where R: DefaultUnsupported<'data> } => x => { x.0 });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_receiver_forward!({ impl<'data, 'a, R: ?Sized> Receiver<'data> for Box<R> where R: Receiver<'data> } => x => { **x });
}
