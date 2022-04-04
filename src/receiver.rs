use crate::{data, Result, Source, Value};

/**
An observer of structured data emitted by some source.

# Data model

Receivers encode `sval`'s data model.

## Text and binary data

Each receiver expects either text-based or binary-based data.
This decision is communicated by [`Receiver::is_text_based`].
Some [data types](#data-types) may be streamed differently depending on whether a receiver is text-based or binary-based.

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
- **Dynamic**: make values heterogenous. See [`Receiver::dynamic_begin`].

All other data types map onto this basic model somehow.

### Extended data types

Receivers may opt-in to direct support for data types in the extended data model either as an optimization, or to handle the differently.
The extended data model includes:

- **Integers**: `i8`-`i128`, `u8`-`u128` and arbitrarily sized. See [`Receiver::int_begin`] and [integer encoding](#integer-encoding).
- **Binary floating points**: `f32`-`f64` and arbitrarily sized. See [`Receiver::binfloat_begin`] and [binary floating point encoding](#binary-floating-point-encoding).
- **Decimal floating points**: These don't have a native Rust counterpart. See [`Receiver::decfloat_begin`] and [decimal floating point encoding](#decimal-floating-point-encoding).

## Values

A value is the sequence of calls that represent a complete [data type](#data-types).
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
    */
    fn is_text_based(&self) -> bool {
        true
    }

    fn value<V: Value + ?Sized + 'data>(&mut self, value: &'data V) -> Result {
        value.stream(self)
    }

    fn unit(&mut self) -> Result;

    fn null(&mut self) -> Result;

    fn bool(&mut self, value: bool) -> Result {
        value.then(|| ()).stream_to_end(self)
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    fn text_fragment(&mut self, fragment: &'data str) -> Result {
        self.text_fragment_computed(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> Result;

    fn text_end(&mut self) -> Result;

    fn text(&mut self, value: &'data str) -> Result {
        self.text_begin(Some(value.len()))?;
        self.text_fragment(value)?;
        self.text_end()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    fn binary_fragment(&mut self, fragment: &'data [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result;

    fn binary_end(&mut self) -> Result;

    fn binary(&mut self, value: &'data [u8]) -> Result {
        self.binary_begin(Some(value.len()))?;
        self.binary_fragment(value)?;
        self.binary_end()
    }

    fn u8(&mut self, value: u8) -> Result {
        data::u8_int(value, self)
    }

    fn u16(&mut self, value: u16) -> Result {
        data::u16_int(value, self)
    }

    fn u32(&mut self, value: u32) -> Result {
        data::u32_int(value, self)
    }

    fn u64(&mut self, value: u64) -> Result {
        data::u64_int(value, self)
    }

    fn u128(&mut self, value: u128) -> Result {
        data::u128_int(value, self)
    }

    fn i8(&mut self, value: i8) -> Result {
        data::i8_int(value, self)
    }

    fn i16(&mut self, value: i16) -> Result {
        data::i16_int(value, self)
    }

    fn i32(&mut self, value: i32) -> Result {
        data::i32_int(value, self)
    }

    fn i64(&mut self, value: i64) -> Result {
        data::i64_int(value, self)
    }

    fn i128(&mut self, value: i128) -> Result {
        data::i128_int(value, self)
    }

    fn f32(&mut self, value: f32) -> Result {
        data::f32_number(value, self)
    }

    fn f64(&mut self, value: f64) -> Result {
        data::f64_number(value, self)
    }

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

    fn dynamic_begin(&mut self) -> Result;

    fn dynamic_end(&mut self) -> Result;

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

    fn int_end(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin an arbitrarily sized binary floating point number.

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

    fn binfloat_end(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin an arbitrarily sized decimal floating point number.

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

    fn decfloat_end(&mut self) -> Result {
        Ok(())
    }

    fn app_specific_begin(&mut self, app_specific_id: u128) -> Result {
        let _ = app_specific_id;

        Ok(())
    }

    fn app_specific_end(&mut self, app_specific_id: u128) -> Result {
        let _ = app_specific_id;

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

            fn app_specific_begin(&mut self, app_specific_id: u128) -> Result {
                let $bind = self;
                ($($forward)*).app_specific_begin(app_specific_id)
            }

            fn app_specific_end(&mut self, app_specific_id: u128) -> Result {
                let $bind = self;
                ($($forward)*).app_specific_end(app_specific_id)
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

    fn app_specific_begin(&mut self, _: u128) -> Result {
        crate::error::unsupported()
    }

    fn app_specific_end(&mut self, _: u128) -> Result {
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
