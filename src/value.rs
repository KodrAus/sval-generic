use crate::{result, std::convert::TryInto, stream::DefaultUnsupported, Result, Stream};

/**
An immutable and repeatable source of structured data.

# Using values

Values are a natural way to capture a source of structured data, like a more advanced `Debug`.
Any value can be streamed through a [`Stream`] with a matching lifetime:

```
# fn wrapped<'sval, V: sval::Value, S: sval::Stream<'sval>>(some_value: impl FnOnce() -> &'sval V, some_stream: impl FnOnce() -> S) -> sval::Result {
use sval::Value;

let value: &'sval V = some_value();

let stream: &mut S = some_stream();

// The `stream` method on `Value` accepts a `Stream`
value.stream(stream)?;
# Ok(())
# }
```

Values can be converted into primitive Rust types:

```
# fn wrapped<'sval, V: sval::Value>(some_value: impl FnOnce() -> &'sval V) -> sval::Result {
use sval::Value;

let value: &'sval V = some_value();

// The `to_*` methods on `Value` perform conversions
let text: &'sval str = value.to_text().unwrap_or_default();

# Ok(())
# }
```

# Building values

## Values always produce complete data

A valid implementation of `Value` must produce a complete and valid value so that it can be used in any context a value is expected.

The following is an example of a Rust type that always produces complete and valid data:

```
struct MyMap<'a>(&'a str, i32);

impl<'a> sval::Value for MyMap<'a> {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // VALID: `MyData` produces a complete map
        stream.map_begin(Some(1))?;

        stream.map_key_begin()?;
        stream.value(&self.0)?;
        stream.map_key_end()?;

        stream.map_value_begin()?;
        stream.value(&self.1)?;
        stream.map_value_end()?;

        stream.map_end()
    }
}
```

The following is an example of a Rust type that _doesn't_ always produce complete data:

```
struct MyMap<'a>(&'a str);

impl<'a> sval::Value for MyMap<'a> {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // INVALID: `MyData` produces a partial map
        stream.map_begin(Some(1))?;

        stream.map_key_begin()?;
        stream.value(&self.0)?;
        stream.map_key_end()?;

        stream.map_value_begin()
    }
}
```

The following is an example of a Rust type that _doesn't_ always produce data that's valid in any context:

```
struct MyKey<'a>(&'a str);

impl<'a> sval::Value for MyKey<'a> {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // INVALID: a map key isn't valid in all contexts
        stream.map_key_begin()?;
        stream.value(&self.0)?;
        stream.map_key_end()
    }
}
```

## Values always map to the same data type

A valid implementation of `Value` must correspond to a single `sval` [data type](trait.Stream.html#data-types).

The following is an example of a Rust type that always produces the same `sval` data type:

```
struct MyData(bool);

impl sval::Value for MyData {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // VALID: `MyData` produces `bool`
        stream.bool(self.0)
    }
}
```

The following is an example of a Rust type that _doesn't_ always produce the same `sval` [data type](#data-types):

```
struct MyData<'a>(&'a str);

impl<'a> sval::Value for MyData<'a> {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // INVALID: `MyData` might produce `bool` or `i64`
        match self.0 {
            "true" => stream.bool(true),
            "false" => stream.bool(false),
            s => stream.i64(s.parse()?),
        }
    }
}
```

This implementation is invalid and needs to be wrapped.
It can be fixed by wrapping the `stream` implementation in dynamic:

```
# struct MyData<'a>(&'a str);
impl<'a> sval::Value for MyData<'a> {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // VALID: `MyData` produces `dynamic`
        stream.dynamic_begin()?;

        match self.0 {
            "true" => stream.bool(true)?,
            "false" => stream.bool(false)?,
            s => stream.i64(s.parse()?)?,
        }

        stream.dynamic_end()
    }
}
```

Wrapping in an enum is also valid:

```
# struct MyData<'a>(&'a str);
impl<'a> sval::Value for MyData<'a> {
    fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, stream: &mut S) -> sval::Result {
        // VALID: `MyData` produces an enum with structural variants
        stream.enum_begin(None)?;
        stream.tagged_begin(None)?;

        match self.0 {
            "true" => stream.bool(true)?,
            "false" => stream.bool(false)?,
            s => stream.i64(s.parse()?)?,
        }

        stream.tagged_end(None)?;
        stream.enum_end(None)
    }
}
```

Data types that wrap others, like dynamic, constant, and fixed size, are order-dependent.

This value:

```
# fn wrap<'a>(stream: impl sval::Stream<'a>) -> sval::Result {
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
# fn wrap<'a>(stream: impl sval::Stream<'a>) -> sval::Result {
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

## Rust structures in `sval`

`sval` can represent the basic building blocks of Rust datastructures.

### Structs with named fields

The struct:

```
struct Struct {
    a: i32,
    b: bool,
}
```

is streamed as a record:

```
# struct Struct { a: i32, b: bool }
# fn wrap<'a>(value: &'a Struct, stream: impl sval::Stream<'a>) -> sval::Result {
stream.record_begin(Some(sval::Tag::Named { name: "Struct", id: None }), Some(2))?;

    stream.record_value_begin(sval::TagNamed { name: "a", id: Some(0) })?;
    stream.value(&value.a)?;
    stream.record_value_end(sval::TagNamed { name: "a", id: Some(0) })?;

    stream.record_value_begin(sval::TagNamed { name: "b", id: Some(1) })?;
    stream.value(&value.b)?;
    stream.record_value_end(sval::TagNamed { name: "b", id: Some(1) })?;

stream.record_end(Some(sval::Tag::Named { name: "Struct", id: None }), Some(2))?;
# Ok(())
# }
```

See [`Stream::record_begin`].

### Structs with unnamed fields

The struct:

```
struct Struct(i32, bool);
```

is streamed as a tuple:

```
# struct Struct(i32, bool);
# fn wrap<'a>(value: &'a Struct, stream: impl sval::Stream<'a>) -> sval::Result {
stream.tuple_begin(Some(sval::Tag::Named { name: "Struct", id: None }), Some(2))?;

    stream.tuple_value_begin(sval::TagUnnamed { id: 0 })?;
    stream.value(&value.0)?;
    stream.tuple_value_end(sval::TagUnnamed { id: 0 })?;

    stream.tuple_value_begin(sval::TagUnnamed { id: 1 })?;
    stream.value(&value.1)?;
    stream.tuple_value_end(sval::TagUnnamed { id: 1 })?;

stream.tuple_end(Some(sval::Tag::Named { name: "Struct", id: None }), Some(2))?;
# Ok(())
# }
```

See [`Stream::tuple_begin`].

### Tuples

```
type Tuple = (i32, bool);
```

## Exotic structures in `sval`

`sval` can represent some data types that Rust can't natively (yet).
These examples just use a strawman Rust syntax to illustrate the shape of the data type in a familiar setting.

#### Anonymous structs with named fields

```rust,ignore
type Record = { a: i32, b: bool };
```

#### Anonymous enums

```rust,ignore
type Enum = (i32 | bool);
```

```
# #![allow(non_camel_case_types)]
# enum Enum { i32(i32), bool(bool) }
# fn wrap<'a>(value: &'a Enum, stream: impl sval::Stream<'a>) -> sval::Result {
# use Enum::*;
stream.enum_begin(None)?;

    stream.tagged_begin(None)?;
    match value {
        i32(i) => stream.value(i)?,
        bool(b) => stream.value(b)?,
    }
    stream.tagged_end(None)?;

stream.enum_end(None)?;
# Ok(())
# }
```

#### Nested enums

```rust,ignore
enum Enum {
    A(i32),
    B(bool),
    enum Nested {
        A(i32),
        B(bool),
    },
}
```
*/
pub trait Value {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result;

    #[inline]
    fn is_dynamic(&self) -> bool {
        struct Check(bool);

        impl<'sval> DefaultUnsupported<'sval> for Check {
            fn dynamic_begin(&mut self) -> Result {
                self.0 = true;
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut check = Check(false).into_stream();
        if let Ok(()) = self.stream(&mut check) {
            (check.0).0
        } else {
            false
        }
    }

    #[inline]
    fn to_bool(&self) -> Option<bool> {
        struct Extract(Option<bool>);

        impl<'sval> DefaultUnsupported<'sval> for Extract {
            fn bool(&mut self, value: bool) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract(None).into_stream();
        self.stream(&mut extract).ok()?;
        (extract.0).0
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        struct Extract(Option<f32>);

        impl<'sval> DefaultUnsupported<'sval> for Extract {
            fn f32(&mut self, value: f32) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract(None).into_stream();
        self.stream(&mut extract).ok()?;
        (extract.0).0
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        struct Extract(Option<f64>);

        impl<'sval> DefaultUnsupported<'sval> for Extract {
            fn f64(&mut self, value: f64) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract(None).into_stream();
        self.stream(&mut extract).ok()?;
        (extract.0).0
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        struct Extract(Option<i128>);

        impl<'sval> DefaultUnsupported<'sval> for Extract {
            fn i128(&mut self, value: i128) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract(None).into_stream();
        self.stream(&mut extract).ok()?;
        (extract.0).0
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        struct Extract(Option<u128>);

        impl<'sval> DefaultUnsupported<'sval> for Extract {
            fn u128(&mut self, value: u128) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract(None).into_stream();
        self.stream(&mut extract).ok()?;
        (extract.0).0
    }

    #[inline]
    fn to_text(&self) -> Option<&str> {
        struct Extract<'sval> {
            extracted: Option<&'sval str>,
            seen_fragment: bool,
        }

        impl<'sval> DefaultUnsupported<'sval> for Extract<'sval> {
            fn text_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> Result {
                // Allow either independent strings, or fragments of a single borrowed string
                if !self.seen_fragment {
                    self.extracted = Some(fragment);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                result::unsupported()
            }

            fn text_end(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }

            fn constant_size_begin(&mut self) -> Result {
                Ok(())
            }

            fn constant_size_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        }
        .into_stream();

        self.stream(&mut extract).ok()?;
        extract.0.extracted
    }

    #[inline]
    fn to_binary(&self) -> Option<&[u8]> {
        struct Extract<'sval> {
            extracted: Option<&'sval [u8]>,
            seen_fragment: bool,
        }

        impl<'sval> DefaultUnsupported<'sval> for Extract<'sval> {
            fn binary_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
                // Allow either independent bytes, or fragments of a single borrowed byte stream
                if !self.seen_fragment {
                    self.extracted = Some(fragment);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                result::unsupported()
            }

            fn binary_end(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_begin(&mut self) -> Result {
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_begin(&mut self) -> Result {
                Ok(())
            }

            fn optional_some_end(&mut self) -> Result {
                Ok(())
            }

            fn constant_size_begin(&mut self) -> Result {
                Ok(())
            }

            fn constant_size_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        }
        .into_stream();

        self.stream(&mut extract).ok()?;
        extract.0.extracted
    }
}

macro_rules! impl_value_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
                let $bind = self;
                ($($forward)*).stream(stream)
            }

            #[inline]
            fn is_dynamic(&self) -> bool {
                let $bind = self;
                ($($forward)*).is_dynamic()
            }

            #[inline]
            fn to_bool(&self) -> Option<bool> {
                let $bind = self;
                ($($forward)*).to_bool()
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                let $bind = self;
                ($($forward)*).to_f32()
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                let $bind = self;
                ($($forward)*).to_f64()
            }

            #[inline]
            fn to_i8(&self) -> Option<i8> {
                let $bind = self;
                ($($forward)*).to_i8()
            }

            #[inline]
            fn to_i16(&self) -> Option<i16> {
                let $bind = self;
                ($($forward)*).to_i16()
            }

            #[inline]
            fn to_i32(&self) -> Option<i32> {
                let $bind = self;
                ($($forward)*).to_i32()
            }

            #[inline]
            fn to_i64(&self) -> Option<i64> {
                let $bind = self;
                ($($forward)*).to_i64()
            }

            #[inline]
            fn to_i128(&self) -> Option<i128> {
                let $bind = self;
                ($($forward)*).to_i128()
            }

            #[inline]
            fn to_u8(&self) -> Option<u8> {
                let $bind = self;
                ($($forward)*).to_u8()
            }

            #[inline]
            fn to_u16(&self) -> Option<u16> {
                let $bind = self;
                ($($forward)*).to_u16()
            }

            #[inline]
            fn to_u32(&self) -> Option<u32> {
                let $bind = self;
                ($($forward)*).to_u32()
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                let $bind = self;
                ($($forward)*).to_u64()
            }

            #[inline]
            fn to_u128(&self) -> Option<u128> {
                let $bind = self;
                ($($forward)*).to_u128()
            }

            #[inline]
            fn to_text(&self) -> Option<&str> {
                let $bind = self;
                ($($forward)*).to_text()
            }

            #[inline]
            fn to_binary(&self) -> Option<&[u8]> {
                let $bind = self;
                ($($forward)*).to_binary()
            }
        }
    };
}

impl_value_forward!({impl<'a, T: Value + ?Sized> Value for &'a T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_value_forward!({impl<T: Value + ?Sized> Value for Box<T>} => x => { **x });
}
