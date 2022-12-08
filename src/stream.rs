use crate::{data, Label, Result, Tag, Index, Value};

/**
An observer of structured data emitted by some value.

# Using streams

Streams can be used to convert between structured data and some text or binary encoding.
They can also be used to observe and transform data as it's yielded by values.

## Borrowing

Streams may accept text and binary data that's borrowed for a particular lifetime (`'sval`).
Borrowing is just an optimization though, and streams also need to expect data that's computed on-demand.

Callers should prefer passing borrowed data where possible.

# Building streams

## Streams are flat

This trait doesn't directly support recursion for streaming nested data like maps and sequences.
Instead, it surrounds nested data with `begin`/`end` calls that remind a stream what context it's in.

## Streams don't validate

Streams aren't responsible for validating the correctness of the data they're given. That's up to
the caller (usually an implementation of [`Value`]) to do. Since these APIs are safe, a `Stream`
can't rely on correctness for memory safety, but are free to fail, panic, or produce garbage when
given invalid input.

## Streams preserve semantics when forwarding

If a stream is forwarding to another it should make an effort to forward all methods accurately,
unless it's specifically transforming the data in some way.

# Data model

Streams encode `sval`'s data model. A data model is like a type system for data. It defines the rules
for what kinds of data can be represented, and how that data can be interpreted. For more details on
specific [data types](#data-types) in the model, see the methods on this trait.

`sval`'s data model isn't a one-to-one mapping of Rust's. It's designed for cases where the consumer
of structured data may be for a different language altogether so gives formats more tools for retaining
the semantics of streamed data.

## Data types

Data types represent the distinct kinds of values that a stream may choose to interpret or encode in
a particular way. If two values have the same data type then a stream is expected to handle them in
compatible ways, even if their content is different. The type definition of a data type specifies
the information that determines whether two values have the same data type or not.

As an example, `u8` and `u16` have different data types, even though Rust will freely coerce between
them, because a `Stream` may rely on their size when encoding them. On the other hand, the data type
of a map does not depend on its size, so a stream is expected to handle maps of any length equivalently.

The docs for each data type call out what is and isn't considered part of the type definition.

### Basic data types

The required methods on the `Stream` trait represent the basic data model that all streams need to 
understand. The basic data model is:

- **Simple values**:
    - **Null**: the absence of any other meaningful value.
- **Encoded data**:
    - **Text blobs**: UTF8 strings.
    - **Binary blobs**: arbitrary byte strings.
- **Complex values**:
    - **Sequences**: homogeneous collection of elements, where elements are [values](#values). All
    elements have the same data type.

All other data types map into this basic model.

### Extended data types

Streams may opt-in to direct support for data types in the extended data model either as an
optimization, or to handle them differently. The extended data model adds:

- **Simple values**:
    - **Unit**: a marker for a value with no other meaningful data. This type is distinct from null. Rust's `()`.
    - **Booleans**: the values `true` and `false`. Rust's `bool`.
    - **Integers**: native integers. Rust's `i8`-`i128`, `u8`-`u128`.
    - **Binary floating points**: native base2 fractional numbers. Rust's `f32`-`f64`.
- **Complex values**:
    - **Maps**: homogeneous collection of key-value pairs, where keys and values are each [values](#values).
    All keys have the same data type, and all values have the same data type.
- **Encoded values**:
    - **Decimal numbers**: Arbitrarily sized decimal numbers with representations for NaNs and infinities.
- **Typed complex values**:
    - **Tagged values**: associate a tag with a [value](#values) so that its data type is distinct 
    from the value type of its underlying data.
    - **Records**: associate tags and labels with a structure and each of its values. Record values 
    are heterogeneous.
    - **Tuples**: associate tags with a structure and each of its values. Tuples values are heterogeneous.
- **Dynamically typed values**:
    - **Dynamic**: make [values](#values) heterogeneous by signaling that they may have any type.
    - **Enums**: make [values](#values) heterogeneous by tagging them as one of a number of 
    non-overlapping variants.
- **Dependently typed values**:
    - **Constant values**: for [values](#values) that will always produce exactly the same data.
    - **Constant sized**: for [values](#values) that will always produce data with exactly the same length.

Streams may opt-in to any parts of the extended data model and ignore the others.

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

## Tags

Some data types accept a tag that associates a label and id with their data. Tag labels are purely 
informational and intended for end-users. Tag ids uniquely identify values either as enum variants,
or as a specialized instance of a data type.

As an example, consider these Rust types:

```
type Tuple = (i32, bool);

struct A(i32, bool);
```

You can think of `A` as a tuple with `i32` and `bool` fields, just like `(i32, bool)`, but `A` and
`(i32, bool)` are not the same tuple. The type of `A` depends on its identifier:

```compile_fail
# type Tuple = (i32, bool);
# struct A(i32, bool);
let t: Tuple = (42, true);

// Does not compile: `Tuple` and `A` are different types
let a: A = t;
```

In `sval`, `A` and `(i32, bool)` can be represented as:

```
# fn wrap<'a>(mut stream: impl sval::Stream<'a>) -> sval::Result {
# fn some_uuid() -> [u8; 16] { Default::default() }
// type Tuple
// This type has a structural tag, so any other tuples that also have structural tags
// and the same fields will match. The `None` field is an informational label, which we don't have
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
stream.tuple_begin(sval::Tag::Tagentified(sval::Tag::new(some_uuid()), Some(sval::Label::new("A"))))?;

stream.tuple_value_begin(0)?;
stream.i32(42)?;
stream.tuple_value_end(0)?;

stream.tuple_value_begin(1)?;
stream.bool(true)?;
stream.tuple_value_end(1)?;

stream.tuple_end(sval::Tag::Tagentified(sval::Tag::new(some_uuid()), Some(sval::Label::new("A"))))?;
# Ok(())
# }
```

The presence of an [`Tag`](../struct.Tag.html) in the tag marks `A` as being a different kind of tuple
as `Tuple`.

When generating code, `sval` won't assign ids to types like `A` on its own. It's up to
implementors to decide if they want this uniqueness property or not.
*/
pub trait Stream<'sval> {
    fn null(&mut self) -> Result;

    fn bool(&mut self, value: bool) -> Result;

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    fn text_fragment(&mut self, fragment: &'sval str) -> Result {
        self.text_fragment_computed(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> Result;

    fn text_end(&mut self) -> Result;

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
        self.seq_begin(num_bytes_hint)
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
        for byte in fragment {
            self.seq_value_begin()?;
            byte.stream(self)?;
            self.seq_value_end()?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> Result {
        self.seq_end()
    }

    fn u8(&mut self, value: u8) -> Result {
        data::number::u8_int(value, self)
    }

    fn u16(&mut self, value: u16) -> Result {
        if let Ok(value) = value.try_into() {
            self.u8(value)
        } else {
            data::number::u16_int(value, self)
        }
    }

    fn u32(&mut self, value: u32) -> Result {
        if let Ok(value) = value.try_into() {
            self.u16(value)
        } else {
            data::number::u32_int(value, self)
        }
    }

    fn u64(&mut self, value: u64) -> Result {
        if let Ok(value) = value.try_into() {
            self.u32(value)
        } else {
            data::number::u64_int(value, self)
        }
    }

    fn u128(&mut self, value: u128) -> Result {
        if let Ok(value) = value.try_into() {
            self.u64(value)
        } else {
            data::number::u128_int(value, self)
        }
    }

    fn i8(&mut self, value: i8) -> Result {
        data::number::i8_int(value, self)
    }

    fn i16(&mut self, value: i16) -> Result {
        if let Ok(value) = value.try_into() {
            self.i8(value)
        } else {
            data::number::i16_int(value, self)
        }
    }

    fn i32(&mut self, value: i32) -> Result {
        if let Ok(value) = value.try_into() {
            self.i16(value)
        } else {
            data::number::i32_int(value, self)
        }
    }

    fn i64(&mut self, value: i64) -> Result {
        if let Ok(value) = value.try_into() {
            self.i32(value)
        } else {
            data::number::i64_int(value, self)
        }
    }

    fn i128(&mut self, value: i128) -> Result {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            data::number::i128_int(value, self)
        }
    }

    fn f32(&mut self, value: f32) -> Result {
        data::number::f32_number(value, self)
    }

    fn f64(&mut self, value: f64) -> Result {
        data::number::f64_number(value, self)
    }

    fn map_key_begin(&mut self) -> Result;

    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;

    fn map_value_end(&mut self) -> Result;

    fn map_end(&mut self) -> Result;

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    fn seq_value_begin(&mut self) -> Result;

    fn seq_value_end(&mut self) -> Result;

    fn seq_end(&mut self) -> Result;

    fn dynamic_begin(&mut self) -> Result {
        Ok(())
    }

    fn dynamic_end(&mut self) -> Result {
        Ok(())
    }

    fn enum_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.tagged_begin(tag, label, index)?;
        self.dynamic_begin()
    }

    fn enum_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.dynamic_end()?;
        self.tagged_end(tag, label, index)
    }

    fn tagged_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        let _ = tag;

        Ok(())
    }

    fn tagged_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        let _ = tag;

        Ok(())
    }

    fn record_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>, num_entries: Option<usize>) -> Result {
        self.tagged_begin(tag, label, index)?;
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

    fn record_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.map_end()?;
        self.constant_size_end()?;
        self.tagged_end(tag, label, index)
    }

    fn tuple_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>, num_entries: Option<usize>) -> Result {
        self.tagged_begin(tag, label, index)?;
        self.constant_size_begin()?;
        self.seq_begin(num_entries)
    }

    fn tuple_value_begin(&mut self, index: Index) -> Result {
        let _ = index;

        self.seq_value_begin()?;
        self.dynamic_begin()
    }

    fn tuple_value_end(&mut self, index: Index) -> Result {
        let _ = index;

        self.dynamic_end()?;
        self.seq_value_end()
    }

    fn tuple_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.seq_end()?;
        self.constant_size_end()?;
        self.tagged_end(tag, label, index)
    }

    fn constant_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.tagged_begin(tag, label, index)
    }

    fn constant_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.tagged_end(tag, label, index)
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

    /**
    Begin an arbitrarily sized decimal floating point number.

    # Structure

    Arbitrary sized decimal floating points wrap a text or binary blob with the encoding described below.
    A call to `number_begin` must be followed by a call to `number_end` after the floating point value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.number_begin()?;

    stream.text_begin(Some(8))?;
    stream.text_fragment("1333.754")?;
    stream.text_end()?;

    stream.number_end()?;
    # Ok(())
    # }
    ```
    */
    fn number_begin(&mut self) -> Result {
        self.tagged_begin(Some(Tag::Number), None, None)
    }

    /**
    End an arbitrary sized decimal floating point number.

    See [`Stream::number_begin`] for details on arbitrary sized decimal floating points.
     */
    fn number_end(&mut self) -> Result {
        self.tagged_end(Some(Tag::Number), None, None)
    }
}

macro_rules! impl_stream_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn dynamic_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).dynamic_begin()
            }

            fn dynamic_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).dynamic_end()
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

            fn tagged_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag, label, index)
            }

            fn tagged_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tagged_end(tag, label, index)
            }

            fn constant_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).constant_begin(tag, label, index)
            }

            fn constant_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).constant_end(tag, label, index)
            }

            fn record_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>, num_entries: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).record_begin(tag, label, index, num_entries)
            }

            fn record_value_begin(&mut self, label: Label) -> Result {
                let $bind = self;
                ($($forward)*).record_value_begin(label)
            }

            fn record_value_end(&mut self, label: Label) -> Result {
                let $bind = self;
                ($($forward)*).record_value_end(label)
            }

            fn record_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).record_end(tag, label, index)
            }

            fn tuple_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>, num_entries: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).tuple_begin(tag, label, index, num_entries)
            }

            fn tuple_value_begin(&mut self, index: Index) -> Result {
                let $bind = self;
                ($($forward)*).tuple_value_begin(index)
            }

            fn tuple_value_end(&mut self, index: Index) -> Result {
                let $bind = self;
                ($($forward)*).tuple_value_end(index)
            }

            fn tuple_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tuple_end(tag, label, index)
            }

            fn enum_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).enum_begin(tag, label, index)
            }

            fn enum_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).enum_end(tag, label, index)
            }

            fn constant_size_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).constant_size_begin()
            }

            fn constant_size_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).constant_size_end()
            }

            fn number_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).number_begin()
            }

            fn number_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).number_end()
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

    fn dynamic_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn dynamic_end(&mut self) -> Result {
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

    fn number_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn number_end(&mut self) -> Result {
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
