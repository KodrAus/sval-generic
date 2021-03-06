use crate::{data, Label, Result, Tag, Value};

/**
An observer of structured data emitted by some value.

# Using streams

Streams can be used to convert between structured data and a [text or binary format](#text-and-binary-data).
They can also be used to observe and transform data as it's yielded by values.

## Borrowing

Streams may accept text and binary data that's borrowed for a particular lifetime (`'sval`).
Borrowing is just an optimization though, and streams also need to expect data that's computed on-demand.

Callers should prefer passing borrowed data where possible.

## Picking a representation

Streams may be either text-based or binary-based. This decision determines how values that don't have a direct representation in Rust should be encoded.
Text-based streams use human-readable formats, like `"true"`, or `"123"`. Binary-based streams use binary formats, like the byte `0`, or bitstrings.

Callers need to check whether a stream is [text-based or binary-based](#text-and-binary-data) before streaming encoded data.
The following example streams an encoded integer as either text or binary depending on the stream:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
stream.int_begin()?;

    if stream.is_text_based() {
        // The stream is text-based
        // Stream the integer as text
        stream.text_begin(Some(3))?;
        stream.text_fragment("123")?;
        stream.text_end()?;
    } else {
        // The stream is binary-based
        // Stream the integer as binary
        stream.binary_begin(Some(1))?;
        stream.binary_fragment(&[0b01111011])?;
        stream.binary_end()?;
    }

stream.int_end()?;
# Ok(())
# }
```

# Building streams

## Streams are flat

This trait doesn't directly support recursion for streaming nested data like maps and sequences.
Instead, it surrounds nested data with `begin`/`end` calls that remind a stream what context it's in.

## Streams don't validate

Streams aren't responsible for validating the correctness of the data they're given.
That's up to the caller (usually an implementation of [`Value`]) to do.

## Streams preserve semantics when forwarding

If a stream is forwarding to another it should make an effort to forward all methods accurately, unless it's specifically transforming the data in some way.

# Data model

Streams encode `sval`'s data model.
For more details on specific [data types](#data-types) in the model, see the methods on this trait.

`sval`'s data model isn't a one-to-one mapping of Rust's.
It's designed for cases where the consumer of structured data may be for a different language altogether so gives formats more tools for retaining the semantics of streamed data.

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
The type definition of a data type specifies the information that determines whether two values have the same data type or not.

As an example, `u8` and `u16` have different data types, even though Rust will freely coerce between them, because a `Stream` may rely on their size when encoding them.
On the other hand, the data type of a map does not depend on its size, so a stream is expected to handle maps of any length equivalently.

The docs for each data type call out what is and isn't considered part of the type definition.

### Basic data types

The required methods on the `Stream` trait represent the basic data model that all streams need to understand.
The basic data model is:

- **Simple values**:
    - **Null**: the absence of any other meaningful value.
- **Encoded data**:
    - **Text blobs**: UTF8 strings.
    - **Binary blobs**: arbitrary byte strings.
- **Complex values**:
    - **Sequences**: homogeneous collection of elements, where elements are [values](#values).

All other data types map into this basic model.

### Extended data types

Streams may opt-in to direct support for data types in the extended data model either as an optimization, or to handle them differently.
The extended data model adds:

- **Simple values**:
    - **Unit**: a marker for a value with no other meaningful data.
    - **Booleans**: the values `true` and `false`.
    - **Integers**: native integers. `i8`-`i128`, `u8`-`u128`.
    - **Binary floating points**: native base2 fractional numbers. `f32`-`f64`.
    - **Optionals**: [values](#values) that may have some data or none.
- **Complex values**:
    - **Maps**: homogeneous collection of key-value pairs, where keys and values are each [values](#values).
- **Encoded values**:
    - **Integers**: Arbitrarily sized signed integers.
    - **Binary floating points**: Arbitrarily sized base2 fractional numbers.
    - **Decimal floating points**: Arbitrarily sized base10 fractional numbers.
- **Typed complex values**:
    - **Tagged values**: associate a tag with a [value](#values) so that its data type is distinct from the value type of its underlying data.
    - **Records**: associate tags and labels with a structure and each of its values. Record values are heterogeneous.
    - **Tuples**: associate tags with a structure and each of its values. Tuples values are heterogeneous.
- **Dynamically typed values**:
    - **Dynamic**: make [values](#values) heterogeneous so that maps and sequences can contain values of different data types.
    - **Enums**: make [values](#values) heterogeneous by tagging them as one of a number of non-overlapping variants.
- **Dependently typed values**:
    - **Constant values**: for [values](#values) that will always have the same data.
    - **Constant sized**: for [values](#values) with a length where that length will always be the same.

### Tags

Some data types accept a tag that associates a label and id with their values.
Tag labels are purely informational and intended for end-users.
Tag ids uniquely identify values either as enum variants, or as a specialized instance of a data type.

As an example, consider these Rust types:

```
type Tuple = (i32, bool);

struct A(i32, bool);
```

You can think of `A` as a tuple with `i32` and `bool` fields, just like `(i32, bool)`, but it's not the same tuple.
The type of `A` depends on its identifier:

```compile_fail
# type Tuple = (i32, bool);
# struct A(i32, bool);
let t: Tuple = (42, true);

// Does not compile: `Tuple` and `A` are different types
let a: A = t;
```

In `sval`, that example might look something like this:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
# fn some_uuid() -> [u8; 16] { Default::default() }
// type Tuple
// This type has a structural tag, so any other tuples that also have structural tags
// and the same fields will match. The `None` field is an informational label
stream.tuple_begin(sval::Tag::Structural(None))?;

    stream.tuple_value_begin(0)?;
    stream.i32(42)?;
    stream.tuple_value_end(0)?;

    stream.tuple_value_begin(1)?;
    stream.bool(true)?;
    stream.tuple_value_end(1)?;

stream.tuple_end(sval::Tag::Structural(None))?;

// struct A
// This type has an identified tag, so any time this UUID is seen, it means this exact type
stream.tuple_begin(sval::Tag::Identified(sval::Id::new(some_uuid()), Some(sval::Label::new("A"))))?;

    stream.tuple_value_begin(0)?;
    stream.i32(42)?;
    stream.tuple_value_end(0)?;

    stream.tuple_value_begin(1)?;
    stream.bool(true)?;
    stream.tuple_value_end(1)?;

stream.tuple_end(sval::Tag::Identified(sval::Id::new(some_uuid()), Some(sval::Label::new("A"))))?;
# Ok(())
# }
```

The presence of an [`Id`](../struct.Id.html) in the tag marks `A` as being a different kind of tuple as `Tuple`.
When generating code, `sval` won't assign identifiers to types like `A` on its own. It's up to implementors to
decide if they want this uniqueness property in their scenario.

Ids on enum variants can be used to distinguish between variants that otherwise have the same content, like:

```
enum MyEnum {
    A(i32),
    B(i32),
}
```

Ids on other values can be used to deduplicate them in schema-based formats.

## Values

A value is the sequence of calls on a stream that represent a single instance of a [data type](trait.Stream.html#data-types).
This concept is distinct from the [`Value`] trait, although it does represent a value in the data model.
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
*/
pub trait Stream<'sval> {
    /**
    Whether or not the stream expects text or binary data.

    This choice is expected to be constant over a single complete value.
    Callers are expected to check this method before choosing between the text or binary encoding for a particular [data type](#data-type).
    */
    fn is_text_based(&self) -> bool {
        true
    }

    /**
    A value that simply _is_.

    Unit is one of the [extended data types](extended-data-types).

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

    # Type equality

    Unit is a distinct data type that only matches other units.
    That means unit and null are not the same data type, and unit and other values like `i32` are not the same data type.

    # Value equality

    All instances of unit are equal.

    # Unit encoding

    For both text-based and binary-based streams unit maps to `null`.
    */
    fn unit(&mut self) -> Result {
        self.null()
    }

    /**
    A value that simply _isn't_.

    Null is one of the [basic data types](basic-data-types), but isn't commonly used directly.
    In Rust, you'd usually use `Option` to represent nullable values.
    `Option::None` doesn't map directly to null though, it maps to an optional.
    See [`Stream::optional_none`] for details.

    # Examples

    Stream a null:

    ```
    # fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.null()?;
    # Ok(())
    # }
    ```

    # Type equality

    Null is a distinct data type that only matches other nulls.
    That means unit and null are not the same data type.

    Rust doesn't have a primitive type that maps directly to null.

    # Value equality

    All instances of null are equal.
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

    # Type equality

    Boolean is a distinct data type that only matches other booleans.
    The values `true` and `false` do have the same data type.

    # Value equality

    Booleans are equal when their values are logically equal.

    # Boolean encoding

    For [text-based streams](#text-and-binary-data), booleans map to the case insensitive text blob `true` or `false`.

    For [binary-based streams](#binary-based-streams), booleans map to a single byte `1` for `true` and `0` for `false`.
    */
    fn bool(&mut self, value: bool) -> Result {
        data::bool_basic(value, self)
    }

    /**
    Begin a UTF8 text blob.

    Text blobs are one of the [basic data types](basic-data-types).
    Most other data types map to text blobs for [text-based streams](text-and-binary-data), but binary-based streams may also stream text.

    The `num_bytes_hint` argument is a hint for how many bytes (not characters) the text blob will contain.
    If a hint is given it must accurately reflect the total number of bytes in the blob.
    A stream should be able to tell whether a single fragment covers the whole blob if its length is equal to this hint.

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

    # Type equality

    All text blobs have the same [data type](#data-types) regardless of length or how they're split into fragments.

    # Value equality

    Text blobs are considered equal when their underlying bytes are equal, regardless of how those bytes are split into fragments.

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
    fn text_fragment(&mut self, fragment: &'sval str) -> Result {
        self.text_fragment_computed(fragment)
    }

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
    If a hint is given it must accurately reflect the total number of bytes in the blob.
    A stream should be able to tell whether a single fragment covers the whole blob if its length is equal to this hint.

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

    # Type equality

    All binary blobs have the same [data type](#data-types) regardless of length or how they're split into fragments.

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
    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

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

    # Type equality

    `u8` is a distinct data type that only matches other `u8`s.
    That means `u8` doesn't have the same type as `i8`, `u16`, or arbitrary sized integers.

    # `u8` encoding

    `u8`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn u8(&mut self, value: u8) -> Result {
        data::number::u8_int(value, self)
    }

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

    # Type equality

    `u16` is a distinct data type that only matches other `u16`s.
    That means `u16` doesn't have the same type as `i16`, `u8`, or arbitrary sized integers.

    # `u16` encoding

    `u16`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn u16(&mut self, value: u16) -> Result {
        if let Ok(value) = value.try_into() {
            self.u8(value)
        } else {
            data::number::u16_int(value, self)
        }
    }

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

    # Type equality

    `u32` is a distinct data type that only matches other `u32`s.
    That means `u32` doesn't have the same type as `i32`, `f32`, `u64`, or arbitrary sized integers.

    # `u32` encoding

    `u32`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn u32(&mut self, value: u32) -> Result {
        if let Ok(value) = value.try_into() {
            self.u16(value)
        } else {
            data::number::u32_int(value, self)
        }
    }

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

    # Type equality

    `u64` is a distinct data type that only matches other `u64`s.
    That means `u64` doesn't have the same type as `i64`, `f64`, `u128`, or arbitrary sized integers.

    # `u64` encoding

    `u64`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn u64(&mut self, value: u64) -> Result {
        if let Ok(value) = value.try_into() {
            self.u32(value)
        } else {
            data::number::u64_int(value, self)
        }
    }

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

    # Type equality

    `u128` is a distinct data type that only matches other `u128`s.
    That means `u128` doesn't have the same type as `i128` or arbitrary sized integers.

    # `u128` encoding

    `u128`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn u128(&mut self, value: u128) -> Result {
        if let Ok(value) = value.try_into() {
            self.u64(value)
        } else {
            data::number::u128_int(value, self)
        }
    }

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

    # Type equality

    `i8` is a distinct data type that only matches other `i8`s.
    That means `i8` doesn't have the same type as `u8`, `i16`, or arbitrary sized integers.

    # `i8` encoding

    `i8`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn i8(&mut self, value: i8) -> Result {
        data::number::i8_int(value, self)
    }

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

    # Type equality

    `i16` is a distinct data type that only matches other `i16`s.
    That means `i16` doesn't have the same type as `u16`, `i8`, or arbitrary sized integers.

    # `i16` encoding

    `i16`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn i16(&mut self, value: i16) -> Result {
        if let Ok(value) = value.try_into() {
            self.i8(value)
        } else {
            data::number::i16_int(value, self)
        }
    }

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

    # Type equality

    `i32` is a distinct data type that only matches other `i32`s.
    That means `i32` doesn't have the same type as `u32`, `f32`, `i64`, or arbitrary sized integers.

    # `i32` encoding

    `i32`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn i32(&mut self, value: i32) -> Result {
        if let Ok(value) = value.try_into() {
            self.i16(value)
        } else {
            data::number::i32_int(value, self)
        }
    }

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

    # Type equality

    `i64` is a distinct data type that only matches other `i64`s.
    That means `i64` doesn't have the same type as `u64`, `f64`, `i128`, or arbitrary sized integers.

    # `i64` encoding

    `i64`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn i64(&mut self, value: i64) -> Result {
        if let Ok(value) = value.try_into() {
            self.i32(value)
        } else {
            data::number::i64_int(value, self)
        }
    }

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

    # Type equality

    `i128` is a distinct data type that only matches other `i128`s.
    That means `i128` doesn't have the same type as `u128` or arbitrary sized integers.

    # `i128` encoding

    `i128`s map to the basic data model as a text or binary blob containing an integer.
    See [`Stream::int_begin`] for more details.
    */
    fn i128(&mut self, value: i128) -> Result {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            data::number::i128_int(value, self)
        }
    }

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

    # Type equality

    `f32` is a distinct data type that only matches other `f32`s.
    That means `f32` doesn't have the same type as `i32`, `f64`, or arbitrary sized floating points.

    # `f32` encoding

    `f32`s map to the basic data model as a text or binary blob containing a binary floating point number.
    See [`Stream::binfloat_begin`] for more details.
    */
    fn f32(&mut self, value: f32) -> Result {
        data::number::f32_number(value, self)
    }

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

    # Type equality

    `f64` is a distinct data type that only matches other `f64`s.
    That means `f64` doesn't have the same type as `f32`, or arbitrary sized floating points.

    # `f64` encoding

    `f64`s map to the basic data model as a text or binary blob containing a binary floating point number.
    See [`Stream::binfloat_begin`] for more details.
    */
    fn f64(&mut self, value: f64) -> Result {
        data::number::f64_number(value, self)
    }

    /**
    Begin a homogeneous map of key-value pairs.

    Maps are one of the [extended data types](extended-data-types).

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
    # fn wrap<'a>(key_values: &'a [(impl Value, impl Value)], mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.map_begin(None)?;

    // Maps contain 0 or more key-value pairs
    for (key, value) in key_values {
        // Keys are a value surrounded by `map_key_begin` and `map_key_end`
        stream.map_key_begin()?;

        // Keys must contain a single value
        key.stream(&mut stream)?;

        stream.map_key_end()?;

        // Values are a value surrounded by `map_value_begin` and `map_value_end`
        // Values must follow keys and all keys must be followed by a value
        stream.map_value_begin()?;

        // Values must contain a single value
        value.stream(&mut stream)?;

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

    # Type equality

    Maps have the same [data type](data-types) as other maps where the data types of their keys and values match, regardless of their length.

    # Value equality

    Maps are considered equal if they have the same length and their key-value pairs (including duplicates) are equal and in the same order.

    # Maps and structs

    Types defined as Rust `struct`s with named fields can be more semantically represented as "struct maps".
    See the [`Stream::record_begin`] method for details.

    # Map encoding

    Maps are encoded in the basic model as a sequence, with each key-value pair encoded as a tuple
    of key and value.
    */
    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
        self.seq_begin(num_entries_hint)
    }

    /**
    Begin a map key.

    See [`Stream::map_begin`] for more details.

    # Type equality

    Map keys are a positional element and aren't considered a data type on their own.
    */
    fn map_key_begin(&mut self) -> Result {
        self.seq_value_begin()?;

        // Encode the key-value pair as a tuple, since keys and values must have
        // the same type.
        self.tuple_begin(Tag::Structural(None), Some(2))?;
        self.tuple_value_begin(0)
    }

    /**
    Complete a map key.
    */
    fn map_key_end(&mut self) -> Result {
        self.tuple_value_end(0)
    }

    /**
    Begin a map value.

    See [`Stream::map_begin`] for more details.

    # Type equality

    Map values are a positional element and aren't considered a data type on their own.
    */
    fn map_value_begin(&mut self) -> Result {
        self.tuple_value_begin(1)
    }

    /**
    Complete a map value.
    */
    fn map_value_end(&mut self) -> Result {
        self.tuple_value_end(1)?;
        self.tuple_end(Tag::Structural(None))?;

        self.seq_value_end()
    }

    /**
    Complete a map.
    */
    fn map_end(&mut self) -> Result {
        self.seq_end()
    }

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
    See [`Stream::constant_size_begin`] for details.

    # Structure

    Sequences must contain zero or more values, followed by a call to [`Stream::seq_end`].

    ```
    # use sval::Value;
    # fn wrap<'a>(values: &'a [impl Value], mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.seq_begin(None)?;

    // Maps contain 0 or more key-value pairs
    for value in values {
        // Values are a value surrounded by `seq_value_begin` and `seq_value_end`
        stream.seq_value_begin()?;

        // Values must contain a single value
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

    # Type equality

    Sequences have the same [data type](data-types) as other sequences where the data types of their values match, regardless of their length.

    # Sequences and structs

    Types defined as Rust `struct`s with unnamed fields can be more semantically represented as "struct sequences".
    See the [`Stream::tuple_begin`] method for details.
    */
    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    /**
    Begin a sequence value.

    See [`Stream::seq_begin`] for more details.

    # Type equality

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

    fn dynamic_begin(&mut self) -> Result {
        Ok(())
    }

    fn dynamic_end(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin a tagged value.

    # Structure

    Enums wrap a tagged value, which represent one of a number of possible variants.

    Variants are distinguished purely by their type, so enums can contain untagged variants.
    Ids must be non-overlapping, so if an id is associated with values of one type, that same id can't be re-used for values of a different type.
    */
    fn enum_begin(&mut self, tag: Tag) -> Result {
        self.tagged_begin(tag)?;
        self.dynamic_begin()
    }

    fn enum_end(&mut self, tag: Tag) -> Result {
        self.dynamic_end()?;
        self.tagged_end(tag)
    }

    fn tagged_begin(&mut self, tag: Tag) -> Result {
        let _ = tag;

        Ok(())
    }

    fn tagged_end(&mut self, tag: Tag) -> Result {
        let _ = tag;

        Ok(())
    }

    fn record_begin(&mut self, tag: Tag, num_entries: Option<usize>) -> Result {
        self.tagged_begin(tag)?;
        self.constant_size_begin()?;
        self.map_begin(num_entries)
    }

    fn record_value_begin(&mut self, label: Label) -> Result {
        self.map_key_begin()?;

        if let Some(label) = label.try_get_static() {
            label.stream(&mut *self)?;
        } else {
            self.text_begin(Some(label.len()))?;
            self.text_fragment_computed(&label)?;
            self.text_end()?;
        }

        self.map_key_end()?;

        self.map_value_begin()?;
        self.dynamic_begin()
    }

    fn record_value_end(&mut self, label: Label) -> Result {
        let _ = label;

        self.dynamic_end()?;
        self.map_value_end()
    }

    fn record_end(&mut self, tag: Tag) -> Result {
        self.map_end()?;
        self.constant_size_end()?;
        self.tagged_end(tag)
    }

    fn tuple_begin(&mut self, tag: Tag, num_entries: Option<usize>) -> Result {
        self.tagged_begin(tag)?;
        self.constant_size_begin()?;
        self.seq_begin(num_entries)
    }

    fn tuple_value_begin(&mut self, index: u32) -> Result {
        let _ = index;

        self.seq_value_begin()?;
        self.dynamic_begin()
    }

    fn tuple_value_end(&mut self, index: u32) -> Result {
        let _ = index;

        self.dynamic_end()?;
        self.seq_value_end()
    }

    fn tuple_end(&mut self, tag: Tag) -> Result {
        self.seq_end()?;
        self.constant_size_end()?;
        self.tagged_end(tag)
    }

    fn constant_begin(&mut self, tag: Tag) -> Result {
        self.tagged_begin(tag)
    }

    fn constant_end(&mut self, tag: Tag) -> Result {
        self.tagged_end(tag)
    }

    /**
    Begin a fixed-size value.

    Data types of variable size that accept an optional hint as a `usize` don't consider that size to be part of their type definition.
    Wrapping values of those data types in fixed-size requires all instances to always have the same size.
    Different sizes become different data types.

    The meaning of size depends on the data type being wrapped.

    # Structure

    Fixed-size values wrap a value of variable size.
    That includes any data type that accepts an optional size hint as a `usize`, such as:

    - Text and binary blobs.
    - Maps.
    - Sequences.

    The actual size of that wrapped value must match the size specified.

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.constant_size_begin()?;

    // The hint and actual size of the binary fragment must be 16
    stream.binary_begin(Some(16))?;
    stream.binary_fragment(&[
        0xa1, 0xa2, 0xa3, 0xa4,
        0xb1, 0xb2,
        0xc1, 0xc2,
        0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ])?;
    stream.binary_end()?;

    stream.constant_size_end()?;
    # Ok(())
    # }
    ```
    */
    fn constant_size_begin(&mut self) -> Result {
        Ok(())
    }

    fn constant_size_end(&mut self) -> Result {
        Ok(())
    }

    fn optional_some_begin(&mut self) -> Result {
        // NOTE: We use `dynamic` here instead of mapping out an `enum`
        // If it was an enum then `optional` would have to be supported as
        // a kind of enum variant
        self.dynamic_begin()
    }

    fn optional_some_end(&mut self) -> Result {
        self.dynamic_end()
    }

    fn optional_none(&mut self) -> Result {
        self.dynamic_begin()?;
        self.null()?;
        self.dynamic_end()
    }

    /**
    Begin an arbitrarily sized integer.

    # Structure

    Arbitrary sized integers wrap a text or binary blob with the encoding described below.
    A call to `int_begin` must be followed by a call to `int_end` after the integer value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.int_begin()?;

    // Integers must contain a single text or binary blob
    if stream.is_text_based() {
        // Text-based streams require a single text blob
        stream.text_begin(Some(3))?;
        stream.text_fragment("754")?;
        stream.text_end()?;
    } else {
        // Binary-based streams require a single binary blob
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
    fn int_begin(&mut self) -> Result {
        Ok(())
    }

    /**
    End an arbitrary sized integer.

    See [`Stream::int_begin`] for details on arbitrary sized integers.
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
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.binfloat_begin()?;

    // Floating points must contain a single text or binary blob
    if stream.is_text_based() {
        // Text-based streams require a single text blob
        stream.text_begin(Some(8))?;
        stream.text_fragment("1333.754")?;
        stream.text_end()?;
    } else {
        // Binary-based streams require a single binary blob
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
    fn binfloat_begin(&mut self) -> Result {
        Ok(())
    }

    /**
    End an arbitrary sized binary floating point number.

    See [`Stream::binfloat_begin`] for details on arbitrary sized binary floating points.
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
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.decfloat_begin()?;

    // Floating points must contain a single text or binary blob
    if stream.is_text_based() {
        // Text-based streams require a single text blob
        stream.text_begin(Some(8))?;
        stream.text_fragment("1333.754")?;
        stream.text_end()?;
    } else {
        // Binary-based streams require a single binary blob
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

    For [binary-based streams](#text-and-binary-data), decimal floating points map to little-endian IEEE754 interchange decimal floating points using the [densely-packed-decimal](https://en.wikipedia.org/wiki/Densely_tuple_decimal) representation.

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

    See [`Stream::decfloat_begin`] for details on arbitrary sized decimal floating points.
     */
    fn decfloat_end(&mut self) -> Result {
        Ok(())
    }
}

macro_rules! impl_stream_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn is_text_based(&self) -> bool {
                let $bind = self;
                ($($forward)*).is_text_based()
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

            fn tagged_begin(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag)
            }

            fn tagged_end(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).tagged_end(tag)
            }

            fn constant_begin(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).constant_begin(tag)
            }

            fn constant_end(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).constant_end(tag)
            }

            fn record_begin(&mut self, tag: Tag, num_entries: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).record_begin(tag, num_entries)
            }

            fn record_value_begin(&mut self, label: Label) -> Result {
                let $bind = self;
                ($($forward)*).record_value_begin(label)
            }

            fn record_value_end(&mut self, label: Label) -> Result {
                let $bind = self;
                ($($forward)*).record_value_end(label)
            }

            fn record_end(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).record_end(tag)
            }

            fn tuple_begin(&mut self, tag: Tag, num_entries: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).tuple_begin(tag, num_entries)
            }

            fn tuple_value_begin(&mut self, index: u32) -> Result {
                let $bind = self;
                ($($forward)*).tuple_value_begin(index)
            }

            fn tuple_value_end(&mut self, index: u32) -> Result {
                let $bind = self;
                ($($forward)*).tuple_value_end(index)
            }

            fn tuple_end(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).tuple_end(tag)
            }

            fn enum_begin(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).enum_begin(tag)
            }

            fn enum_end(&mut self, tag: Tag) -> Result {
                let $bind = self;
                ($($forward)*).enum_end(tag)
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

            fn constant_size_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).constant_size_begin()
            }

            fn constant_size_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).constant_size_end()
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
    fn into_stream(self) -> IntoStream<Self>
    where
        Self: Sized,
    {
        IntoStream(self)
    }

    fn is_text_based(&self) -> bool {
        false
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

    fn tagged_begin(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn tagged_end(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn constant_begin(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn constant_end(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn record_begin(&mut self, _: Tag, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn record_value_begin(&mut self, _: Label) -> Result {
        crate::result::unsupported()
    }

    fn record_value_end(&mut self, _: Label) -> Result {
        crate::result::unsupported()
    }

    fn record_end(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn tuple_begin(&mut self, _: Tag, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn tuple_value_begin(&mut self, _: u32) -> Result {
        crate::result::unsupported()
    }

    fn tuple_value_end(&mut self, _: u32) -> Result {
        crate::result::unsupported()
    }

    fn tuple_end(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn enum_begin(&mut self, _: Tag) -> Result {
        crate::result::unsupported()
    }

    fn enum_end(&mut self, _: Tag) -> Result {
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

    fn constant_size_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn constant_size_end(&mut self) -> Result {
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

pub(crate) struct IntoStream<T: ?Sized>(pub(crate) T);

impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for &'a mut S where S: Stream<'sval> } => x => { **x });
impl_stream_forward!({ impl<'sval, 'a, S> Stream<'sval> for IntoStream<S> where S: DefaultUnsupported<'sval> } => x => { x.0 });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for Box<S> where S: Stream<'sval> } => x => { **x });
}
