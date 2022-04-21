use crate::{Result, Tag, Value};

/**
An observer of structured data emitted by some value.

# Using streams

Streams can be used to convert between structured data and a [text or binary format](text-and-binary-data).
They can also be used to observe and transform data as it's yielded by values.

# Data model

Streams encode `sval`'s data model.

## Text and binary data

Each stream expects either text-based or binary-based data.
This decision is communicated by [`Stream::is_text_based`].
Some [data types](#data-types) may be streamed differently depending on whether a stream is text-based or binary-based.

Streams should only ever expect [values](#values) encoded using either the text or binary representation for their [data type](#data-types).
Binary-based streams may still receive text and text-based streams may still receive binary though.
This means `sval` effectively has two in-memory representations of its data model: one for text-based and one for binary-based streams.

## Data types

Data types represent the distinct kinds of data that a stream may choose to interpret or encode in a particular way.
If two values have the same data type then a stream is expected to handle them in compatible ways, even if their content is different.
As an example, `u8` and `u16` have different data types, even though Rust will freely coerce between them, because a `Stream` may rely on their size when encoding them.
On the other hand, the data type of maps does not depend on their size, so a stream is expected to handle maps of any length equivalently.

### Basic data types

The required methods on the `Stream` trait represent the basic data model that all streams need to understand.
The basic data model includes:

- **Unit**: the truthy value. See [`Stream::unit`].
- **Null**: the falsey value. See [`Stream::null`].
- **Text blobs**: UTF8 strings. See [`Stream::text_begin`].
- **Binary blobs**: arbitrary byte strings. See [`Stream::binary_begin`].
- **Maps**: homogeneous collection of key-value pairs, where keys and values are each [values](#values). See [`Stream::map_begin`].
- **Sequences**: homogeneous collection of values, where elements are [values](#values). See [`Stream::seq_begin`].

All other data types map onto this basic model somehow.

### Extended data types

Streams may opt-in to direct support for data types in the extended data model either as an optimization, or to handle them differently.
The extended data model includes:

- **Booleans**: the values `true` and `false`. See [`Stream::bool`].
- **Integers**: `i8`-`i128`, `u8`-`u128` and arbitrarily sized. See [`Stream::int_begin`] and [integer encoding](#integer-encoding).
- **Binary floating points**: `f32`-`f64` and arbitrarily sized. See [`Stream::binfloat_begin`] and [binary floating point encoding](#binary-floating-point-encoding).
- **Decimal floating points**: These don't have a native Rust counterpart. See [`Stream::decfloat_begin`] and [decimal floating point encoding](#decimal-floating-point-encoding).
- **Dynamic**: make [values](#values) heterogeneous so that maps and sequences can contain values of different data types. See [`Stream::dynamic_begin`].
- **Enums**: make [values](#values) heterogeneous by tagging them as one of a number of non-overlapping variants. See [`Stream::enum_begin`].

#### Wrapping

Data types that wrap others, like dynamic, constant, and fixed size, are order-dependent.

This value:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
// This value...
stream.dynamic_begin()?;
stream.constant_begin()?;

stream.i32(42)?;

stream.constant_end()?;
stream.dynamic_end()?;
# Ok(())
# }
```

does not have the same data type as this one:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
// ...does not match this one
stream.constant_begin()?;
stream.dynamic_begin()?;

stream.i32(42)?;

stream.dynamic_end()?;
stream.constant_end()?;
# Ok(())
# }
```

The first is a dynamic value that happens to contain a constant.
The second is a constant that holds a dynamic value.
This restriction isn't entirely necessary, but it aims to simplify stream encoding.

## Values

A value is the sequence of calls that represent a complete instance of a [data type](#data-types).
The following are all examples of values.

A single integer:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
stream.i32(42)?;
# Ok(())
# }
```

A text blob, streamed as a list of fragments:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
stream.text_begin(Some(14))?;

stream.text_fragment("A blob ")?;
stream.text_fragment("of text")?;

stream.text_end()?;
# Ok(())
# }
```

A map of text-integer key-value pairs:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
stream.map_begin(Some(2))?;

stream.map_key_begin()?;
stream.text_begin(Some(1))?;
stream.text_fragment("a")?;
stream.text_end()?;
stream.map_key_end()?;

stream.map_value_begin()?;
stream.i32(1)?;
stream.map_value_end()?;

stream.map_key_begin()?;
stream.text_begin(Some(1))?;
stream.text_fragment("b")?;
stream.text_end()?;
stream.map_key_end()?;

stream.map_value_begin()?;
stream.i32(2)?;
stream.map_value_end()?;

stream.map_end()?;
# Ok(())
# }
```

A stream should expect just one value over its lifetime.

## Validation

Streams aren't responsible for validating the correctness of the data they're given.
That's up to the caller to do.

## Forwarding

If a stream is forwarding to another it should make an effort to forward all methods accurately, unless it's specifically transforming the data in some way.

# Borrowing

Streams may accept text and binary data that's borrowed for a particular lifetime (`'sval`).
Borrowing is just an optimization though, and streams also need to expect data that's short-lived.
*/
pub trait Stream<'sval> {
    /**
    Whether or not the stream expects text or binary data.

    This choice is expected to be constant over a single complete value.
    Callers are expected to check this method before choosing between the text or binary encoding for a particular [data type](#data-type).
    */
    #[cfg(not(test))]
    fn is_text_based(&self) -> bool {
        true
    }

    #[cfg(test)]
    fn is_text_based(&self) -> bool;

    /**
    A borrowed value.

    This is a niche method that simply calls back into the stream, so shouldn't be called from [`Value::stream`].
    It can be useful for separating borrowed data out to avoid needing to buffer it.
    */
    #[cfg(not(test))]
    fn value<V: Value + ?Sized + 'sval>(&mut self, value: &'sval V) -> Result {
        value.stream(self)
    }

    #[cfg(test)]
    fn value<V: Value + ?Sized + 'sval>(&mut self, value: &'sval V) -> Result;

    /**
    A value that simply _is_.

    Unit is one of the [basic data types](basic-data-types), but isn't commonly used directly.

    # Examples

    Stream a unit:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.unit()?;
    # Ok(())
    # }
    ```

    Rust's `()` type also streams as unit:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    ().stream(stream)?;
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
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.null()?;
    # Ok(())
    # }
    ```

    # Data type

    Null is a distinct data type that only matches other nulls.
    That means unit and null are not the same data type.

    Rust doesn't have a primitive type that maps to null.
    The `Option` type will stream its `None` variant as null, but wrapped in a nullable (see [`Stream::optional_some_begin`]) so that it
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
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.bool(true)?;
    # Ok(())
    # }
    ```

    Rust's `bool` type also streams as a boolean:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    true.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    Boolean is a distinct data type that only matches other booleans.
    The values `true` and `false` do have the same data type.

    # Boolean encoding

    Booleans map to the basic data model as an empty nullable, so `true` will become unit (see [`Stream::unit`]) and `false` will become null (see [`Stream::null`]).
    Also see [`Stream::optional_some_begin`] for more details.
    */
    #[cfg(not(test))]
    fn bool(&mut self, value: bool) -> Result {
        self.dynamic_begin()?;

        if value {
            self.unit()?;
        } else {
            self.null()?;
        }

        self.dynamic_end()
    }

    #[cfg(test)]
    fn bool(&mut self, value: bool) -> Result;

    /**
    Begin a UTF8 text blob.

    Text blobs are one of the [basic data types](basic-data-types).
    Most other data types map to text blobs for [text-based streams](text-and-binary-data), but binary-based streams may also stream text.

    The `num_bytes_hint` argument is a hint for how many bytes (not characters) the text blob will contain.
    If a hint is given it should be as accurate as possible.

    # Examples

    Stream a text blob using a single string:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.text_begin(Some(14))?;

    stream.text_fragment("A blob of text")?;

    stream.text_end()?;
    # Ok(())
    # }
    ```

    Rust's `str` type also streams as a text blob:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    "A blob of text".stream(stream)?;
    # Ok(())
    # }
    ```

    # Structure

    After beginning a text blob, the stream should only expect zero or more text fragments ([`Stream::text_fragment`] or [`Stream::text_fragment_computed`]) followed by a call to [`Stream::text_end`]:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.text_begin(num_bytes_hint)?;

    // 0 or more calls to any combination of text_fragment and text_fragment_computed

    stream.text_end()?;
    # Ok(())
    # }
    ```

    # Borrowing

    Text blobs may contain data that's borrowed for the stream's `'sval` lifetime.
    Fragments streamed using [`Stream::text_fragment`] will be borrowed for `'sval`.
    Fragments streamed using [`Stream::text_fragment_computed`] will be arbitrarily short-lived.

    Callers should use data borrowed for `'sval` wherever possible.
    Borrowing is just an optimization though, so streams need to cater to both cases.

    The following example uses [`Stream::text_fragment_computed`] to stream a blob of computed text:

    ```
    # fn compute_text() -> String { Default::default() }
    # fn wrap<'a>(borrowed_text: &'a str, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.text_begin(None)?;

    // This borrowed text lives for `'sval`
    stream.text_fragment(borrowed_text)?;

    // This owned text only lives until the end of our function call
    // So we need to stream it as a computed fragment
    let s: String = compute_text();
    stream.text_fragment_computed(&s)?;

    stream.text_end()?;
    # Ok(())
    # }
    ```
    */
    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    /**
    A UTF8 text fragment that's borrowed for `'sval`.

    See [`Stream::text_begin`] for details on text fragments.
    The [`Stream::text_fragment_computed`] method is an alternative to this one that doesn't need to borrow for `'sval`.
    */
    #[cfg(not(test))]
    fn text_fragment(&mut self, fragment: &'sval str) -> Result {
        self.text_fragment_computed(fragment)
    }

    #[cfg(test)]
    fn text_fragment(&mut self, fragment: &'sval str) -> Result;

    /**
    A UTF8 text fragment that's borrowed for some arbitrarily short lifetime.

    See [`Stream::text_begin`] for details on text fragments.
    The [`Stream::text_fragment`] method is an alternative to this one that borrows for `'sval`.
    */
    fn text_fragment_computed(&mut self, fragment: &str) -> Result;

    /**
    End a UTF8 text blob.

    See [`Stream::text_begin`] for details on text fragments.
    */
    fn text_end(&mut self) -> Result;

    /**
    Begin a binary blob.

    Binary blobs are one of the [basic data types](basic-data-types).
    Most other data types map to binary blobs for [binary-based streams](text-and-binary-data), but text-based streams may also stream binary.

    The `num_bytes_hint` argument is a hint for how many bytes the binary blob will contain.
    If a hint is given it should be as accurate as possible.

    # Examples

    Stream a binary blob using a single string:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.binary_begin(Some(5))?;

    stream.binary_fragment(&[0xaa, 0xbb, 0xcc, 0xdd, 0x00])?;

    stream.binary_end()?;
    # Ok(())
    # }
    ```

    # Structure

    After beginning a binary blob, the stream should only expect zero or more binary fragments ([`Stream::binary_fragment`] or [`Stream::binary_fragment_computed`]) followed by a call to [`Stream::binary_end`]:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.binary_begin(num_bytes_hint)?;

    // 0 or more calls to any combination of binary_fragment and binary_fragment_computed

    stream.binary_end()?;
    # Ok(())
    # }
    ```

    # Borrowing

    Binary blobs may contain data that's borrowed for the stream's `'sval` lifetime.
    Fragments streamed using [`Stream::binary_fragment`] will be borrowed for `'sval`.
    Fragments streamed using [`Stream::binary_fragment_computed`] will be arbitrarily short-lived.

    Callers should use data borrowed for `'sval` wherever possible.
    Borrowing is just an optimization though, so streams need to cater to both cases.

    The following example uses [`Stream::binary_fragment_computed`] to stream a blob of computed binary:

    ```
    # fn compute_binary() -> Vec<u8> { Default::default() }
    # fn wrap<'a>(borrowed_binary: &'a [u8], mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.binary_begin(None)?;

    // This borrowed binary lives for `'sval`
    stream.binary_fragment(borrowed_binary)?;

    // This owned binary only lives until the end of our function call
    // So we need to stream it as a computed fragment
    let s: Vec<u8> = compute_binary();
    stream.binary_fragment_computed(&s)?;

    stream.binary_end()?;
    # Ok(())
    # }
    ```
    */
    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    /**
    A binary fragment that's borrowed for `'sval`.

    See [`Stream::binary_begin`] for details on binary fragments.
    The [`Stream::binary_fragment_computed`] method is an alternative to this one that doesn't need to borrow for `'sval`.
    */
    #[cfg(not(test))]
    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

    #[cfg(test)]
    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result;

    /**
    A binary fragment that's borrowed for some arbitrarily short lifetime.

    See [`Stream::binary_begin`] for details on binary fragments.
    The [`Stream::binary_fragment`] method is an alternative to this one that borrows for `'sval`.
    */
    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result;

    /**
    End a binary blob.

    See [`Stream::binary_begin`] for details on binary fragments.
    */
    fn binary_end(&mut self) -> Result;

    /**
    Stream an 8bit unsigned integer.

    `u8`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u8`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.u8(42)?;
    # Ok(())
    # }
    ```

    Rust's `u8` type also streams as an 8bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42u8.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `u8` is a distinct data type that only matches other `u8`s.
    That means `u8` doesn't have the same type as `i8`, `u16`, or arbitrary sized integers.

    # `u8` encoding

    `u8`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn u8(&mut self, value: u8) -> Result {
        crate::data::number::u8_int(value, self)
    }

    #[cfg(test)]
    fn u8(&mut self, value: u8) -> Result;

    /**
    Stream a 16bit unsigned integer.

    `u16`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u16`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.u16(42)?;
    # Ok(())
    # }
    ```

    Rust's `u16` type also streams as a 16bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42u16.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `u16` is a distinct data type that only matches other `u16`s.
    That means `u16` doesn't have the same type as `i16`, `u8`, or arbitrary sized integers.

    # `u16` encoding

    `u16`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn u16(&mut self, value: u16) -> Result {
        if let Ok(value) = value.try_into() {
            self.u8(value)
        } else {
            crate::data::number::u16_int(value, self)
        }
    }

    #[cfg(test)]
    fn u16(&mut self, value: u16) -> Result;

    /**
    Stream a 32bit unsigned integer.

    `u32`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u32`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.u32(42)?;
    # Ok(())
    # }
    ```

    Rust's `u32` type also streams as a 32bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42u32.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `u32` is a distinct data type that only matches other `u32`s.
    That means `u32` doesn't have the same type as `i32`, `f32`, `u64`, or arbitrary sized integers.

    # `u32` encoding

    `u32`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn u32(&mut self, value: u32) -> Result {
        if let Ok(value) = value.try_into() {
            self.u16(value)
        } else {
            crate::data::number::u32_int(value, self)
        }
    }

    #[cfg(test)]
    fn u32(&mut self, value: u32) -> Result;

    /**
    Stream a 64bit unsigned integer.

    `u64`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u64`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.u64(42)?;
    # Ok(())
    # }
    ```

    Rust's `u64` type also streams as a 64bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42u64.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `u64` is a distinct data type that only matches other `u64`s.
    That means `u64` doesn't have the same type as `i64`, `f64`, `u128`, or arbitrary sized integers.

    # `u64` encoding

    `u64`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn u64(&mut self, value: u64) -> Result {
        if let Ok(value) = value.try_into() {
            self.u32(value)
        } else {
            crate::data::number::u64_int(value, self)
        }
    }

    #[cfg(test)]
    fn u64(&mut self, value: u64) -> Result;

    /**
    Stream a 128bit unsigned integer.

    `u128`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `u128`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.u128(42)?;
    # Ok(())
    # }
    ```

    Rust's `u128` type also streams as a 128bit unsigned integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42u128.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `u128` is a distinct data type that only matches other `u128`s.
    That means `u128` doesn't have the same type as `i128` or arbitrary sized integers.

    # `u128` encoding

    `u128`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn u128(&mut self, value: u128) -> Result {
        if let Ok(value) = value.try_into() {
            self.u64(value)
        } else {
            crate::data::number::u128_int(value, self)
        }
    }

    #[cfg(test)]
    fn u128(&mut self, value: u128) -> Result;

    /**
    Stream an 8bit signed integer.

    `i8`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i8`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.i8(42)?;
    # Ok(())
    # }
    ```

    Rust's `i8` type also streams as an 8bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42i8.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `i8` is a distinct data type that only matches other `i8`s.
    That means `i8` doesn't have the same type as `u8`, `i16`, or arbitrary sized integers.

    # `i8` encoding

    `i8`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn i8(&mut self, value: i8) -> Result {
        crate::data::number::i8_int(value, self)
    }

    #[cfg(test)]
    fn i8(&mut self, value: i8) -> Result;

    /**
    Stream a 16bit signed integer.

    `i16`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i16`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.i16(42)?;
    # Ok(())
    # }
    ```

    Rust's `i16` type also streams as a 16bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42i16.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `i16` is a distinct data type that only matches other `i16`s.
    That means `i16` doesn't have the same type as `u16`, `i8`, or arbitrary sized integers.

    # `i16` encoding

    `i16`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn i16(&mut self, value: i16) -> Result {
        if let Ok(value) = value.try_into() {
            self.i8(value)
        } else {
            crate::data::number::i16_int(value, self)
        }
    }

    #[cfg(test)]
    fn i16(&mut self, value: i16) -> Result;

    /**
    Stream a 32bit signed integer.

    `i32`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i32`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.i32(42)?;
    # Ok(())
    # }
    ```

    Rust's `i32` type also streams as a 32bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42i32.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `i32` is a distinct data type that only matches other `i32`s.
    That means `i32` doesn't have the same type as `u32`, `f32`, `i64`, or arbitrary sized integers.

    # `i32` encoding

    `i32`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn i32(&mut self, value: i32) -> Result {
        if let Ok(value) = value.try_into() {
            self.i16(value)
        } else {
            crate::data::number::i32_int(value, self)
        }
    }

    #[cfg(test)]
    fn i32(&mut self, value: i32) -> Result;

    /**
    Stream a 64bit signed integer.

    `i64`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i64`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.i64(42)?;
    # Ok(())
    # }
    ```

    Rust's `i64` type also streams as a 64bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42i64.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `i64` is a distinct data type that only matches other `i64`s.
    That means `i64` doesn't have the same type as `u64`, `f64`, `i128`, or arbitrary sized integers.

    # `i64` encoding

    `i64`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn i64(&mut self, value: i64) -> Result {
        if let Ok(value) = value.try_into() {
            self.i32(value)
        } else {
            crate::data::number::i64_int(value, self)
        }
    }

    #[cfg(test)]
    fn i64(&mut self, value: i64) -> Result;

    /**
    Stream a 128bit signed integer.

    `i128`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream an `i128`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.i128(42)?;
    # Ok(())
    # }
    ```

    Rust's `i128` type also streams as a 128bit signed integer:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    42i128.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `i128` is a distinct data type that only matches other `i128`s.
    That means `i128` doesn't have the same type as `u128` or arbitrary sized integers.

    # `i128` encoding

    `i128`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    #[cfg(not(test))]
    fn i128(&mut self, value: i128) -> Result {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            crate::data::number::i128_int(value, self)
        }
    }

    #[cfg(test)]
    fn i128(&mut self, value: i128) -> Result;

    /**
    Stream a 32bit binary floating number.

    `f32`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `f32`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.f32(4.2)?;
    # Ok(())
    # }
    ```

    Rust's `f32` type also streams as a 32bit binary floating number:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    4.2f32.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `f32` is a distinct data type that only matches other `f32`s.
    That means `f32` doesn't have the same type as `i32`, `f64`, or arbitrary sized floating points.

    # `f32` encoding

    `f32`s map to the basic data model as a text or binary blob containing a binary floating point number.
    See [`Stream::binfloat_begin`] for more details.
    */
    #[cfg(not(test))]
    fn f32(&mut self, value: f32) -> Result {
        crate::data::number::f32_number(value, self)
    }

    #[cfg(test)]
    fn f32(&mut self, value: f32) -> Result;

    /**
    Stream a 64bit binary floating number.

    `f64`s are one of the [extended data types](extended-data-types).

    # Examples

    Stream a `f64`:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.f64(4.2)?;
    # Ok(())
    # }
    ```

    Rust's `f64` type also streams as a 64bit binary floating number:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    4.2f64.stream(stream)?;
    # Ok(())
    # }
    ```

    # Data type

    `f64` is a distinct data type that only matches other `f64`s.
    That means `f64` doesn't have the same type as `f32`, or arbitrary sized floating points.

    # `f64` encoding

    `f64`s map to the basic data model as a text or binary blob containing a binary floating point number.
    See [`Stream::binfloat_begin`] for more details.
    */
    #[cfg(not(test))]
    fn f64(&mut self, value: f64) -> Result {
        crate::data::number::f64_number(value, self)
    }

    #[cfg(test)]
    fn f64(&mut self, value: f64) -> Result;

    /**
    Begin a homogeneous map of key-value pairs.

    Maps are one of the [basic data types](basic-data-types).

    The `num_entries_hint` parameter is an optional hint for the number of pairs the map will contain.
    If a hint is given it should be accurate, but streams can't rely on the correctness of any hints.

    # Examples

    Stream some key-value pairs as a map:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.map_begin(Some(2))?;

    stream.map_key_begin()?;
    stream.text_begin(Some(2))?;

    stream.text_fragment("id")?;

    stream.text_end()?;
    stream.map_key_end()?;

    stream.map_value_begin()?;
    stream.text_begin(Some(5))?;

    stream.text_fragment("An id")?;

    stream.text_end()?;
    stream.map_value_end()?;

    stream.map_key_begin()?;
    stream.text_begin(Some(5))?;

    stream.text_fragment("title")?;

    stream.text_end()?;
    stream.map_key_end()?;

    stream.map_value_begin()?;
    stream.text_begin(Some(10))?;

    stream.text_fragment("A document")?;

    stream.text_end()?;
    stream.map_value_end()?;

    stream.map_end()?;
    # Ok(())
    # }
    ```

    # Structure

    Maps must contain zero or more pairs of keys and values, followed by a call to [`Stream::map_end`].

    ```
    # use sval::Value;
    # fn wrap<'a>(key_values: &'a [(impl sval::Value, impl sval::Value)], mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.map_begin(None)?;

    // Maps contain 0 or more key-value pairs
    for (key, value) in key_values {
        // Keys are a value surrounded by `map_key_begin` and `map_key_end`
        stream.map_key_begin()?;

        stream.value(key)?;

        stream.map_key_end()?;

        // Values are a value surrounded by `map_value_begin` and `map_value_end`
        // Values must follow keys and all keys must be followed by a value
        stream.map_value_begin()?;

        stream.value(value)?;

        stream.map_value_end()?;
    }

    stream.map_end()?;
    # Ok(())
    # }
    ```

    # Maps are homogeneous

    The [data type](data-types) of all keys and the [data type](data-types) of all values must be the same.

    Maps can contain heterogeneous data if keys and values are dynamic or enums.
    See [`Stream::dynamic_begin`] and [`Stream::enum_begin`] for more details.
    The following example is a map with string keys and dynamic values:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.map_begin(Some(2))?;

    stream.map_key_begin()?;
    stream.text_begin(Some(2))?;

    stream.text_fragment("id")?;

    stream.text_end()?;
    stream.map_key_end()?;

    stream.map_value_begin()?;
    stream.dynamic_begin()?;

    stream.i32(42)?;

    stream.dynamic_end()?;
    stream.map_value_end()?;

    stream.map_key_begin()?;
    stream.text_begin(Some(5))?;

    stream.text_fragment("title")?;

    stream.text_end()?;
    stream.map_key_end()?;

    stream.map_value_begin()?;
    stream.dynamic_begin()?;
    stream.text_begin(Some(10))?;

    stream.text_fragment("A document")?;

    stream.text_end()?;
    stream.dynamic_end()?;
    stream.map_value_end()?;

    stream.map_end()?;
    # Ok(())
    # }
    ```

    # Data type

    Maps have the same [data type](data-types) as other maps where the data types of their keys and values match, regardless of their length.

    # Maps and structs

    Types defined as Rust `struct`s with named fields can be more semantically represented as "struct maps".
    See the [`Stream::struct_map_begin`] method for details.
    */
    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    /**
    Begin a map key.

    See [`Stream::map_begin`] for more details.

    # Data type

    Map keys are a positional element and aren't considered a data type on their own.
    */
    fn map_key_begin(&mut self) -> Result;

    /**
    Complete a map key.
    */
    fn map_key_end(&mut self) -> Result;

    /**
    Begin a map value.

    See [`Stream::map_begin`] for more details.

    # Data type

    Map values are a positional element and aren't considered a data type on their own.
    */
    fn map_value_begin(&mut self) -> Result;

    /**
    Complete a map value.
    */
    fn map_value_end(&mut self) -> Result;

    /**
    Complete a map.
    */
    fn map_end(&mut self) -> Result;

    /**
    Begin a homogeneous sequence of values.

    Sequences are one of the [basic data types](basic-data-types).

    The [data type](data-types) of all values must be the same.

    The `num_entries_hint` parameter is an optional hint for the number of values the sequence will contain.
    If a hint is given it should be accurate, but streams can't rely on the correctness of any hints.

    # Examples

    Stream some values as a sequence:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.seq_begin(Some(2))?;

    stream.seq_value_begin()?;

    stream.i32(1)?;

    stream.seq_value_end()?;

    stream.seq_value_begin()?;

    stream.i32(2)?;

    stream.seq_value_end()?;

    stream.seq_end()?;
    # Ok(())
    # }
    ```

    Rust's unsized array (`[T]`) type is streamed as a sequence:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    let slice: &[i32] = &[1, 2, 3];
    slice.stream(stream)?;
    # Ok(())
    # }
    ```

    Fixed-size arrays (`[T; N]`) are also streamed as sequences:

    ```
    # use sval::Value;
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    let slice: [i32; 3] = [1, 2, 3];
    slice.stream(stream)?;
    # Ok(())
    # }
    ```

    The fact that the size of these arrays is fixed is retained.
    See [`Stream::fixed_size_begin`] for details.

    # Structure

    Sequences must contain zero or more values, followed by a call to [`Stream::seq_end`].

    ```
    # use sval::Value;
    # fn wrap<'a>(values: &'a [impl sval::Value], mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.seq_begin(None)?;

    // Maps contain 0 or more key-value pairs
    for value in values {
        // Values are a value surrounded by `seq_value_begin` and `seq_value_end`
        stream.seq_value_begin()?;
        stream.value(value)?;
        stream.seq_value_end()?;
    }

    stream.seq_end()?;
    # Ok(())
    # }
    ```

    # Sequences are homogeneous

    The [data type](data-types) of all values must be the same.

    Sequences can contain heterogeneous data if values are dynamic or enums.
    See [`Stream::dynamic_begin`] and [`Stream::enum_begin`] for more details.
    The following example is a sequence with dynamic values:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.seq_begin(Some(2))?;

    stream.seq_value_begin()?;
    stream.dynamic_begin()?;

    stream.i32(1)?;

    stream.dynamic_end()?;
    stream.seq_value_end()?;

    stream.seq_value_begin()?;
    stream.dynamic_begin()?;
    stream.text_begin(Some(7))?;

    stream.text_fragment("A value")?;

    stream.text_end()?;
    stream.dynamic_end()?;
    stream.seq_value_end()?;

    stream.seq_end()?;
    # Ok(())
    # }
    ```

    # Data type

    Sequences have the same [data type](data-types) as other sequences where the data types of their values match, regardless of their length.

    # Sequences and structs

    Types defined as Rust `struct`s with unnamed fields can be more semantically represented as "struct sequences".
    See the [`Stream::struct_seq_begin`] method for details.
    */
    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    /**
    Begin a sequence value.

    See [`Stream::seq_begin`] for more details.

    # Data type

    Sequence values are a positional element and aren't considered a data type on their own.
    */
    fn seq_value_begin(&mut self) -> Result;

    /**
    Complete a sequence value.
    */
    fn seq_value_end(&mut self) -> Result;

    /**
    Complete a sequence.
    */
    fn seq_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn dynamic_begin(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn dynamic_begin(&mut self) -> Result;

    #[cfg(not(test))]
    fn dynamic_end(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn dynamic_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn fixed_size_begin(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn fixed_size_begin(&mut self) -> Result;

    #[cfg(not(test))]
    fn fixed_size_end(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn fixed_size_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn tagged_begin(&mut self, tag: Option<Tag>) -> Result {
        let _ = tag;

        Ok(())
    }

    #[cfg(test)]
    fn tagged_begin(&mut self, tag: Option<Tag>) -> Result;

    #[cfg(not(test))]
    fn tagged_end(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn tagged_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn constant_begin(&mut self, tag: Option<Tag>) -> Result {
        self.tagged_begin(tag)
    }

    #[cfg(test)]
    fn constant_begin(&mut self, tag: Option<Tag>) -> Result;

    #[cfg(not(test))]
    fn constant_end(&mut self) -> Result {
        self.tagged_end()
    }

    #[cfg(test)]
    fn constant_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn struct_map_begin(&mut self, tag: Option<Tag>, num_entries_hint: Option<usize>) -> Result {
        self.tagged_begin(tag)?;
        self.map_begin(num_entries_hint)
    }

    #[cfg(test)]
    fn struct_map_begin(&mut self, tag: Option<Tag>, num_entries_hint: Option<usize>) -> Result;

    #[cfg(not(test))]
    fn struct_map_key_begin(&mut self, tag: Tag) -> Result {
        self.map_key_begin()?;
        self.constant_begin(Some(tag))?;
        self.dynamic_begin()
    }

    #[cfg(test)]
    fn struct_map_key_begin(&mut self, tag: Tag) -> Result;

    #[cfg(not(test))]
    fn struct_map_key_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.constant_end()?;
        self.map_key_end()
    }

    #[cfg(test)]
    fn struct_map_key_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn struct_map_value_begin(&mut self, tag: Tag) -> Result {
        let _ = tag;

        self.map_value_begin()?;
        self.dynamic_begin()
    }

    #[cfg(test)]
    fn struct_map_value_begin(&mut self, tag: Tag) -> Result;

    #[cfg(not(test))]
    fn struct_map_value_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.map_value_end()
    }

    #[cfg(test)]
    fn struct_map_value_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn struct_map_end(&mut self) -> Result {
        self.map_end()
    }

    #[cfg(test)]
    fn struct_map_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn struct_seq_begin(&mut self, tag: Option<Tag>, num_entries_hint: Option<usize>) -> Result {
        self.tagged_begin(tag)?;
        self.seq_begin(num_entries_hint)
    }

    #[cfg(test)]
    fn struct_seq_begin(&mut self, tag: Option<Tag>, num_entries_hint: Option<usize>) -> Result;

    #[cfg(not(test))]
    fn struct_seq_value_begin(&mut self, tag: Tag) -> Result {
        let _ = tag;

        self.seq_value_begin()?;
        self.dynamic_begin()
    }

    #[cfg(test)]
    fn struct_seq_value_begin(&mut self, tag: Tag) -> Result;

    #[cfg(not(test))]
    fn struct_seq_value_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.seq_value_end()
    }

    #[cfg(test)]
    fn struct_seq_value_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn struct_seq_end(&mut self) -> Result {
        self.seq_end()
    }

    #[cfg(test)]
    fn struct_seq_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn enum_begin(&mut self, tag: Option<Tag>) -> Result {
        self.tagged_begin(tag)?;
        self.dynamic_begin()
    }

    #[cfg(test)]
    fn enum_begin(&mut self, tag: Option<Tag>) -> Result;

    #[cfg(not(test))]
    fn enum_end(&mut self) -> Result {
        self.dynamic_end()?;
        self.tagged_end()
    }

    #[cfg(test)]
    fn enum_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn optional_some_begin(&mut self) -> Result {
        self.enum_begin(Some(crate::Tag::Labeled {
            label: "Option",
            id: 0,
        }))?;
        self.tagged_begin(Some(crate::Tag::Labeled {
            label: "Some",
            id: 1,
        }))
    }

    #[cfg(test)]
    fn optional_some_begin(&mut self) -> Result;

    #[cfg(not(test))]
    fn optional_some_end(&mut self) -> Result {
        self.tagged_end()?;
        self.enum_end()
    }

    #[cfg(test)]
    fn optional_some_end(&mut self) -> Result;

    #[cfg(not(test))]
    fn optional_none(&mut self) -> Result {
        self.enum_begin(Some(crate::Tag::Labeled {
            label: "Option",
            id: 0,
        }))?;

        self.constant_begin(Some(crate::Tag::Labeled {
            label: "None",
            id: 0,
        }))?;
        self.null()?;
        self.constant_end()?;

        self.enum_end()
    }

    #[cfg(test)]
    fn optional_none(&mut self) -> Result;

    /**
    Begin an arbitrarily sized integer.

    # Structure

    Arbitrary sized integers wrap a text or binary blob with the encoding described below.
    A call to `int_begin` must be followed by a call to `int_end` after the integer value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.int_begin()?;

    if stream.is_text_based() {
        stream.text_begin(Some(3))?;
        stream.text_fragment("754")?;
        stream.text_end()?;
    } else {
        stream.binary_begin(Some(2))?;
        stream.binary_fragment(&[0b11110010, 0b00000010])?;
        stream.binary_end()?;
    }

    stream.int_end()?;
    # Ok(())
    # }
    ```

    # Integer encoding

    Each kind of integer is considered a different data type.
    So `u8` is a different type to `i8` and `u8` is a different type to `u16`.
    All arbitarily sized integers (those streamed using [`Stream::int_begin`]) are considered the same type.

    `i8`-`i128`, `u8`-`u128`, and arbitrary-sized integers use the same text-based or binary-based encoding described below.

    For [text-based streams](#text-and-binary-data), integers map to text blobs representing a base10 number with the following grammar:

    ```text
    -?[0-9]+
    ```

    For [binary-based streams](#binary-based-streams), integers map to signed, little-endian, two's-compliment bytes.

    The following table shows some example integers along with their text and binary encodings.
    The binary encoding uses the smallest possible representation, even though that's not a requirement.

    | Integer | Text encoding | Binary encoding     |
    | ------- | ------------: | ------------------: |
    | 0       | `0`           | `00000000`          |
    | 754     | `754`         | `11110010_00000010` |
    | -754    | `-754`        | `00001110_11111101` |
    */
    #[cfg(not(test))]
    fn int_begin(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn int_begin(&mut self) -> Result;

    /**
    End an arbitrary sized integer.

    See [`Stream::int_begin`] for details on arbitrary sized integers.
    */
    #[cfg(not(test))]
    fn int_end(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn int_end(&mut self) -> Result;

    /**
    Begin an arbitrarily sized binary floating point number.

    # Structure

    Arbitrary sized binary floating points wrap a text or binary blob with the encoding described below.
    A call to `binfloat_begin` must be followed by a call to `binfloat_end` after the floating point value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.binfloat_begin()?;

    if stream.is_text_based() {
        stream.text_begin(Some(8))?;
        stream.text_fragment("1333.754")?;
        stream.text_end()?;
    } else {
        stream.binary_begin(Some(4))?;
        stream.binary_fragment(&[0b00100001, 0b10111000, 0b10100110, 0b01000100])?;
        stream.binary_end()?;
    }

    stream.binfloat_end()?;
    # Ok(())
    # }
    ```

    # Binary floating point encoding

    `f32` is a different type to `f64`.
    All arbitrarily sized binary floating points (those streamed using [`Stream::binfloat_begin`]) are considered the same type, regardless of size.

    `f32`, `f64`, and arbitrarily-sized floating points use the same text-based or binary-based encoding described below.

    For [text-based streams](#text-and-binary-data), binary floating points map to text blobs representing a base10 number with the following case-insensitive grammar:

    ```text
    inf|[-+]?(nan|[0-9]+(\.[0-9]+)?)
    ```

    For [binary-based streams](#text-and-binary-data), binary floating points map to little-endian IEEE754 interchange binary floating points.

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
    #[cfg(not(test))]
    fn binfloat_begin(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn binfloat_begin(&mut self) -> Result;

    /**
    End an arbitrary sized binary floating point number.

    See [`Stream::binfloat_begin`] for details on arbitrary sized binary floating points.
    */
    #[cfg(not(test))]
    fn binfloat_end(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn binfloat_end(&mut self) -> Result;

    /**
    Begin an arbitrarily sized decimal floating point number.

    # Structure

    Arbitrary sized decimal floating points wrap a text or binary blob with the encoding described below.
    A call to `decfloat_begin` must be followed by a call to `decfloat_end` after the floating point value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.decfloat_begin()?;

    if stream.is_text_based() {
        stream.text_begin(Some(8))?;
        stream.text_fragment("1333.754")?;
        stream.text_end()?;
    } else {
        stream.binary_begin(Some(4))?;
        stream.binary_fragment(&[0b1101010, 0b1100111, 0b0010011, 0b00100110])?;
        stream.binary_end()?;
    }

    stream.decfloat_end()?;
    # Ok(())
    # }
    ```

    # Decimal floating point encoding

    Rust doesn't have any native decimal floating point types.
    All arbitrarily sized decimal floating points (those streamed using [`Stream::decfloat_begin`]) are considered the same type.

    For [text-based streams](#text-and-binary-data), decimal floating points use the same encoding as [binary floating points](#binary-floating-point-encoding).

    For [binary-based streams](#text-and-binary-data), decimal floating points map to little-endian IEEE754 interchange decimal floating points using the [densely-packed-decimal](https://en.wikipedia.org/wiki/Densely_packed_decimal) representation.

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
    #[cfg(not(test))]
    fn decfloat_begin(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn decfloat_begin(&mut self) -> Result;

    /**
    End an arbitrary sized decimal floating point number.

    See [`Stream::decfloat_begin`] for details on arbitrary sized decimal floating points.
     */
    #[cfg(not(test))]
    fn decfloat_end(&mut self) -> Result {
        Ok(())
    }

    #[cfg(test)]
    fn decfloat_end(&mut self) -> Result;
}

macro_rules! impl_stream_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn is_text_based(&self) -> bool {
                let $bind = self;
                ($($forward)*).is_text_based()
            }

            fn value<V: Value + ?Sized + 'sval>(&mut self, value: &'sval V) -> Result {
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

            fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).text_begin(num_bytes_hint)
            }

            fn text_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).text_end()
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment(fragment)
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment_computed(fragment)
            }

            fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).binary_begin(num_bytes_hint)
            }

            fn binary_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).binary_end()
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
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

            fn tagged_begin(&mut self, tag: Option<Tag>) -> Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag)
            }

            fn tagged_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).tagged_end()
            }

            fn constant_begin(&mut self, tag: Option<Tag>) -> Result {
                let $bind = self;
                ($($forward)*).constant_begin(tag)
            }

            fn constant_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).constant_end()
            }

            fn struct_map_begin(&mut self, tag: Option<Tag>, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_begin(tag, num_entries_hint)
            }

            fn struct_map_key_begin(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_key_begin(tag)
            }

            fn struct_map_key_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).struct_map_key_end()
            }

            fn struct_map_value_begin(&mut self, tag: Tag) -> Result {
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

            fn struct_seq_begin(&mut self, tag: Option<Tag>, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).struct_seq_begin(tag, num_entries_hint)
            }

            fn struct_seq_value_begin(&mut self, tag: Tag) -> Result {
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

            fn enum_begin(&mut self, tag: Option<Tag>) -> Result {
                let $bind = self;
                ($($forward)*).enum_begin(tag)
            }

            fn enum_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).enum_end()
            }

            fn optional_some_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).optional_some_begin()
            }

            fn optional_some_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).optional_some_end()
            }

            fn optional_none(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).optional_none()
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

// Simplifies the default streams for extracting concrete types from values
pub(crate) trait DefaultUnsupported<'sval> {
    fn as_stream(&mut self) -> AsStream<&mut Self> {
        AsStream(self)
    }

    fn is_text_based(&self) -> bool {
        false
    }

    fn value<V: Value + ?Sized + 'sval>(&mut self, v: &'sval V) -> Result {
        v.stream(self.as_stream())
    }

    fn dynamic_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn dynamic_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn unit(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn null(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn u8(&mut self, _: u8) -> Result {
        crate::result::unsupported()
    }

    fn u16(&mut self, _: u16) -> Result {
        crate::result::unsupported()
    }

    fn u32(&mut self, _: u32) -> Result {
        crate::result::unsupported()
    }

    fn u64(&mut self, _: u64) -> Result {
        crate::result::unsupported()
    }

    fn u128(&mut self, _: u128) -> Result {
        crate::result::unsupported()
    }

    fn i8(&mut self, _: i8) -> Result {
        crate::result::unsupported()
    }

    fn i16(&mut self, _: i16) -> Result {
        crate::result::unsupported()
    }

    fn i32(&mut self, _: i32) -> Result {
        crate::result::unsupported()
    }

    fn i64(&mut self, _: i64) -> Result {
        crate::result::unsupported()
    }

    fn i128(&mut self, _: i128) -> Result {
        crate::result::unsupported()
    }

    fn f32(&mut self, _: f32) -> Result {
        crate::result::unsupported()
    }

    fn f64(&mut self, _: f64) -> Result {
        crate::result::unsupported()
    }

    fn bool(&mut self, _: bool) -> Result {
        crate::result::unsupported()
    }

    fn text_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn text_fragment(&mut self, _: &'sval str) -> Result {
        crate::result::unsupported()
    }

    fn text_fragment_computed(&mut self, _: &str) -> Result {
        crate::result::unsupported()
    }

    fn text_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn binary_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn binary_fragment(&mut self, _: &'sval [u8]) -> Result {
        crate::result::unsupported()
    }

    fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
        crate::result::unsupported()
    }

    fn binary_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn map_key_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_key_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_value_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_value_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn seq_value_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn seq_value_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn seq_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn tagged_begin(&mut self, _: Option<Tag>) -> Result {
        crate::result::unsupported()
    }

    fn tagged_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn constant_begin(&mut self, _: Option<Tag>) -> Result {
        crate::result::unsupported()
    }

    fn constant_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn struct_map_begin(&mut self, _: Option<Tag>, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn struct_map_key_begin(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn struct_map_key_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn struct_map_value_begin(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn struct_map_value_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn struct_map_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn struct_seq_begin(&mut self, _: Option<Tag>, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn struct_seq_value_begin(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn struct_seq_value_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn struct_seq_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn enum_begin(&mut self, _: Option<Tag>) -> Result {
        crate::result::unsupported()
    }

    fn enum_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn optional_some_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn optional_some_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn optional_none(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn fixed_size_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn fixed_size_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn int_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn int_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn binfloat_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn binfloat_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn decfloat_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn decfloat_end(&mut self) -> Result {
        crate::result::unsupported()
    }
}

pub(crate) struct AsStream<T: ?Sized>(T);

impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for &'a mut S where S: Stream<'sval> } => x => { **x });
impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for AsStream<&'a mut S> where S: DefaultUnsupported<'sval> } => x => { x.0 });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for Box<S> where S: Stream<'sval> } => x => { **x });
}
