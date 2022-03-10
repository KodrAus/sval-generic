# `sval`

`sval` is a serialization API for Rust that streams the structure of data like a tokenizer.

The API is made up of a few concepts:

- `Receiver`: An observer of structured data. These use a flat API that receives a sequence of invocations representing the structure of some value, like a boolean, the start of a map, or the end of a sequence.
- `Source`: The source of structured data. It could be an instance of some concrete type, JSON in a byte buffer, data being read from a file, anything.
- `Value`: A special kind of `Source` that can stream its structure in a side-effect-free way. It will probably be an instance of some concrete type.

### Sources and values

Imagine you want to read some bytes from a source. To do that, you can use the `Read` trait. It looks a little something like this:

```rust
pub trait Read {
    fn read(&mut self, into: &mut [u8]) -> Result<usize>;
}
```

You keep calling `Read::read` until you get `Ok(0)`, meaning the byte stream has been exhausted.

The `Source` trait in `sval` is very similar to `Read`, but instead of reading bytes, it reads structure:

```rust
pub trait Source<'a> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Resume>
    where
        'a: 'b;
}
```

You keep calling `Source::stream_resume` until you get `Ok(Resume::Done)`, meaning the data stream has been exhausted.

What does the structure in a source look like? For a concrete example, let's consider this bit of JSON:

```json
{
    "id": 42,
    "title": "My Document",
    "active": true
}
```

A source reading this document could make the following calls to a receiver:

```rust
receiver.map_begin(Some(3))?;        // {

receiver.map_key_begin()?;           // "
receiver.str("id")?;                 // id
receiver.map_key_end()?;             // "

receiver.map_value_begin()?;         // :
receiver.i32(42)?;                   // 42
receiver.map_value_end()?;           // ,

receiver.map_key_begin()?;           // "
receiver.str("title")?;              // title
receiver.map_key_end()?;             // "

receiver.map_value_begin()?;         // :
receiver.str("My Document")?;        // "My Document"
receiver.map_value_end()?;           // ,

receiver.map_key_begin()?;           // "
receiver.str("active")?;             // active
receiver.map_key_end()?;             // "

receiver.map_value_begin()?;         // :
receiver.bool(true)?;                // true
receiver.map_value_end()?;           // 

receiver.map_end()?;                 // }
```

Each call roughly corresponds to a token in the JSON stream. At its core, `sval` is flat to support streaming from a flat source, like a UTF8 encoded text buffer containing a JSON document. Not all sources are flat though, so this same example could also be more compactly written as:

```rust
receiver.map_begin(Some(3))?;

receiver.map_entry("id", 42)?;
receiver.map_entry("title", "My Document")?;
receiver.map_entry("active", true)?;

receiver.map_end()?;
```

This is much closer to how `serde` represents structure in its model.

In `sval`, buffering and borrowing are both optional. You can use them to optimize streaming, but receivers work on flat structure too.

### Data model

`sval`'s core data-model is JSON-like. It supports:

- **Values**:
    - **Null**: A value that simply _isn't_.
    - **Unit**: A value that simply _is_.
    - **Numbers**:
        - **Signed integers**: `i8` to `i128`.
        - **Unsigned integers**: `u8` to `u128`.
        - **Floating point**: `f32` to `f64`.
    - **Booleans**: `true` and `false`.
    - **Text**:
        - **String values**: `str`.
        - **Text blobs**: UTF8 strings of known or unknown length, streamed in fragments.
    - **Binary**:
        - **Byte values**: `[u8]`.
        - **Binary blobs**: Byte sequences of known or unknown length, streamed in fragments.
    - **Maps**: Homogenous mapping of value keys to value data.
    - **Sequences**: Homogenous array of values.

### Type, shape, and tags

In Rust, all values have a _type_ that describes its common properties. In `sval`, all data has a _shape_ that describes its common structure.

The shape of a value is determined by the calls it makes on a `Receiver` while streaming.

`sval` extends its basic data model with the concept of _tags_: in-band annotations that influence the shape and semantics of basic data. As an example, a regular map can be annotated as a struct, which requires all keys are strings:

```rust
receiver.tagged_begin(tag().for_struct().with_label("Data"));
receiver.map_begin(Some(3))?;

receiver.map_entry(
    "id",
    tag().for_struct_field().with_label("id").with_value(42)
)?;

receiver.map_entry(
    "title",
    tag().for_struct_field().with_label("title").with_value("My document")
)?;

receiver.map_entry(
    "active",
    tag().for_struct_field().with_label("active").with_value(true)
)?;

receiver.map_end()?;
receiver.tagged_end(tag().for_struct().with_label("Data"));
```

### Dynamic values

`sval` is strict in its requirement that all `Source`s and `Value`s with the same Rust type produce data with the same `sval` shape.
This is a departure from `serde`'s data model and isn't always desirable, so there's a tag shape that identifies values as changing their shape dynamically:

```rust
receiver.tagged_begin(tag().for_dynamic())?;

if some_condition() {
    receiver.i32(42)?;
} else {
    receiver.bool(false)?;
}

receiver.tagged_end(tag().for_dynamic())?;
```

Dynamic values can also be represented as enums:

```rust
receiver.tagged_begin(tag().for_enum().with_label("Data"))?;

if some_condition() {
    receiver.tagged(tag().with_label("Variant1"), 42)?;
} else {
    receiver.tagged(tag().with_label("Variant2"), false)?;
}

receiver.tagged_end(tag().for_enum().with_label("Data"))?;
```
