# `sval`

This is an experimental redesign of the object-safe serialization framework `sval`. It avoids some of the original's key issues:

- Only supporting trait objects limits suitability. This approach uses a generic core so you can avoid indirection in both runtime and in API design, but still makes it easy to erase to an object-safe wrapper. `sval` can perform in the same ballpark as `serde`.
- Repurposing the name `Stream`, which already has a well-known meaning in Rust as an asynchronous iterator. This approach calls these receivers of structured data `Receiver`s.
- Not being able to represent things like structs and enums natively. This approach uses _tags_ as a concept to annotate maps and sequences with a type or variant.
- Potential method explosion from needing to support values passed as either borrowed for a specific lifetime, borrowed for some arbitrarily-short lifetime, or borrowed for the static lifetime. This approach uses a trait to be generic over the lifetime of the value, allowing callers to get a zero-cost arbitrarily-short reference, or try get one for a specific lifetime.

## What is it?

`sval` is a serialization API for Rust that's designed around surfacing the structure in data like the tokenizer in a parser.

The API is made up of a few concepts:

- `Receiver`: An observer of structured data. These use a flat API that receives a sequence of invocations representing the structure of some value, like a boolean, the start of a map, or the end of a sequence.
- `Source`: The source of structured data. It could be an instance of some concrete type, JSON in a byte buffer, data being read from a file, anything.
- `Value`: A special kind of `Source` that can stream its structure in a side-effect-free way. It will probably be an instance of some concrete type.

## How is it different?

There are a few serialization frameworks out there already:

- `serde`: Designed around separating values from specific formats by standardizing an abstract data-model for Rust objects that all values and formats can agree on. The API is split between converting serializable values into some format and deserializing values from some format.
- `valuable`: Like a reflection API for data. It lets you inspect the shape of Rust objects where metadata is mostly known statically so you can efficiently do things like generically check whether the value of a particular named field exists with a particular type.

The approach `sval` takes is a bit of a mash-up of [`serde`](https://github.com/serde-rs/serde)'s serialize and deserialize APIs into one. You can serialize by connecting a `Source` that represents some concrete type to a `Receiver` that writes it to a buffer in a particular format. You can deserialize by connecting a `Source` that represents a tokenizer for a particular format over a buffer to a `Receiver` that builds up some concrete type. It has a flat base, so data can be streamed without recursion. It also trades some correct-by-construction use of by-value receivers and associated types for easier object-safe wrapping. You can use erased versions of `Source` and `Receiver` without potentially needing an allocator.

There's some overlap in the usecases served by all of these libraries, but each takes a particular set of trade-offs to best serve the needs of their users.
