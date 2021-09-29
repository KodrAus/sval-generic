# API

This is an experimental alternative API for `sval` that avoids some of the costs associated with trait objects and consolidates around a few core traits.

`sval` is like a blend of `serde::ser` and `serde::de`. It:

- splits producers and consumers of structured data into values and streams respectively.
- uses a flat underlying API for streams that all higher-level, value-based APIs forward to.
- uses an opportunistic lifetime parameter to support borrowing when possible.

# Data model

`sval` uses a JSON-like data model that also supports tags.

## Unit structs

A Rust value like:

```rust
struct MyStruct;

MyStruct
```

is streamed as:

```rust
none()?;
```

which produces:

```json
null
```

## Field-value structs

A Rust value like:

```rust
struct MyStruct<'a> {
    a: i32,
    b: bool,
    c: &'a str,
}

MyStruct {
    a: 42,
    b: true,
    c: "a string"
}
```

is streamed as:

```rust
type_tagged_map_begin(type_tag("MyStruct"), Some(3))?;

map_field("a")?;
map_value(42)?;

map_field("b")?;
map_value(true)?;

map_field("c")?;
map_value("a string")?;

type_tagged_map_end()?;
```

which expands to:

```rust
type_tagged_begin(type_tag("MyStruct"))?;
map_begin(Some(3))?;

map_key_begin()?;
str("a")?;
map_key_end()?;

map_value_begin()?;
i64(42)?;
map_value_end()?;

map_key_begin()?;
str("b")?;
map_key_end()?;

map_value_begin()?;
bool(true)?;
map_value_end()?;

map_key_begin()?;
str("c")?;
map_key_end()?;

map_value_begin()?;
str("a string")?;
map_value_end()?;

map_end()?;
type_tagged_end()?;
```

which produces:

```json
{
    "a": 42,
    "b": true,
    "c": "a string"
}
```

## Field-value struct enums

A Rust value like:

```rust
enum MyEnum<'a> {
    MyVariant {
        a: i32,
        b: bool,
        c: &'a str,
    }
}

MyEnum::MyVariant {
    a: 42,
    b: true,
    c: "a string"
}
```

is streamed as:

```rust
variant_tagged_map_begin(variant_tag("MyEnum", "MyVariant", Some(0)))?;

map_field("a")?;
map_value(42)?;

map_field("b")?;
map_value(true)?;

map_field("c")?;
map_value("a string")?;

variant_tagged_map_end()?;
```

which expands to:

```rust
variant_tagged_begin(variant_tag("MyEnum", "MyVariant", Some(0)))?;
map_begin(Some(3))?;

map_key_begin()?;
str("a")?;
map_key_end()?;

map_value_begin()?;
i64(42)?;
map_value_end()?;

map_key_begin()?;
str("b")?;
map_key_end()?;

map_value_begin()?;
bool(true)?;
map_value_end()?;

map_key_begin()?;
str("c")?;
map_key_end()?;

map_value_begin()?;
str("a string")?;
map_value_end()?;

map_end()?;
variant_tagged_end()?;
```

which produces:

```json
{
    "MyVariant": {
        "a": 42,
        "b": true,
        "c": "a string"
    }
}
```

# `Value` and `ValueRef`

These traits seem very similar at first. Their relationship is roughly:

```
&'a impl Value = impl ValueRef<'a>
```

There are other implementors of `ValueRef<'a>` besides `&'a impl Value`:

- Primitive types like `i32` and `bool`. That lets you pass these types by-value to stream methods instead of needing to create a borrow of them.
- `ForAll<impl Value>`. That lets you bypass the `'a` lifetime requirement and send short-lived data through a stream.

When deciding whether you should be using `Value` or `ValueRef`:

- Implement `Value` for your type. You don't need to worry about `ValueRef`.
- Prefer `fn func(v: impl Value)` over `fn func<'a>(v: impl ValueRef<'a>)`.
- Prefer `fn func<'a>(v: impl ValueRef<'a>)` over `fn func<'a>(v: &'a impl Value)`.
