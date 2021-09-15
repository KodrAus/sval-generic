# API

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
struct_map("MyStruct", Some(3), |mut map| {
    map.field("a", 42)?;
    map.field("b", true)?;
    map.field("c", "a string")
})?;
```

which expands to:

```rust
type_tagged_begin("MyStruct")?;
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
{ "a": 42, "b": true, "c": "a string" }
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
struct_variant_map("MyEnum", "MyVariant", Some(0), Some(3), |mut map| {
    map.field("a", 42)?;
    map.field("b", true)?;
    map.field("c", "a string")
})?;
```

which expands to:

```rust
variant_tagged_begin("MyEnum", "MyVariant", Some(0))?;
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
{ "MyVariant": { "a": 42, "b": true, "c": "a string" }}
```
