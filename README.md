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

The approach `sval` takes is a bit of a mash-up of `serde`'s serialize and deserialize APIs into one. You can serialize by connecting a `Source` that represents some concrete type to a `Receiver` that writes it to a buffer in a particular format. You can deserialize by connecting a `Source` that represents a tokenizer for a particular format over a buffer to a `Receiver` that builds up some concrete type. It has a flat base, so data can be streamed without recursion. It also trades some correct-by-construction use of by-value receivers and associated types for easier object-safe wrapping. You can use erased versions of `Source` and `Receiver` without potentially needing an allocator.

There's some overlap in the usecases served by all of these libraries, but each takes a particular set of trade-offs to best serve the needs of their users.

## Generators and coroutines

This repository also has some exploration of different approaches to building a general-purpose resumable serialization API. Rust has a stable `async`/`await` syntax based on unstable generators (`yield`). Unfortunately, `async`/`await` isn't usable everywhere yet so it imposes some significant limitations on API design.

With a resumable serializer, we wouldn't necessarily need direct `async`/`await` support to utilize them later. We can put a bound on how much data to buffer before potentially yielding, and let the `async`-aware wrapper take care of waking up when there's more work we can do. At least that's the theory. The other theory is that using `struct`s and `enum`s as source gives us a limited enough design space that we don't need to try design something as general as Rust's own `async`/`await` or generators.

I've sketched out two approaches in here:

- Using generators: in the `src/generator` module. These are very similar to what Rust's unstable generators produce. You get a state machine where each `yield` is a variant. When you resume, you match on that variant to figure out where you're at. You can nest generators by stashing a child in a variant of its parent. Rust will naturally pack generators tightly and with aggressive optimizations you can flatten them down well.
- Using coroutines: in the `src/coroutine` module. These yield the next piece of the computation to invoke instead of tracking state in `enum`s. The cost of resuming and the depth of the callstack are somewhat fixed regardless of the complexity of the coroutine (and by extension the data it's serializing). A coroutine needs space for its state, which may live on the stack or get boxed to the heap. You can nest coroutines by leaving space for the state of a child in its parent. Rust can still pack this state tightly, but the scope of optimizations is limited.

In general, I've been finding that the coroutine approach performs better than the generator approach for the kinds of serialization workloads I've looked at, and has some nice properties for dealing with very deeply nested data. I think it's worth exploring a resumable serialization API with its internals based entirely off a `#[derive]` macro more fully.

Proportionally here's roughly how things look on my i9 9900K desktop:

```
test twitter_erased_sval_generic_api      ... bench:     296,546 ns/iter (+/- 17,343)
test twitter_sval_generic_api             ... bench:     260,779 ns/iter (+/- 13,690)
test twitter_sval_generic_api_coroutine   ... bench:     338,383 ns/iter (+/- 18,995)
test twitter_sval_generic_api_generator   ... bench:     469,486 ns/iter (+/- 25,373)
```

The tradeoff is that the coroutine approach needs to use `Pin` and unsafe code so that it can properly store interior pointers into its state to resume from (I'm pretty sure the current code will be unsound too).

The code generated for each approach is roughly as follows:

```rust
// Non-resumable
const _: () = {
    impl Value for Twitter {
        fn stream<'a, R: Receiver<'a>>(
            &'a self,
            mut receiver: R,
        ) -> Result {
            receiver
                .type_tagged_map_begin(tag::type_tag("Twitter"), Some(2usize))?;

            receiver.map_field_entry("statuses", &self.statuses)?;
            receiver.map_field_entry("search_metadata", &self.search_metadata)?;

            receiver.type_tagged_map_end()
        }
    }
};
```

```rust
// Using generators
const _: () = {
    impl GeneratorValue for Twitter {
        type Generator<'a> = Generator_Twitter<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            Generator_Twitter {
                value: self,
                generator: GeneratorState_Twitter::Begin,
            }
        }

        fn stream<'a, R: Receiver<'a>>(
            &'a self,
            mut receiver: R,
        ) -> Result {
            receiver
                .type_tagged_map_begin(tag::type_tag("Twitter"), Some(2usize))?;

            receiver.map_field_entry(
                "statuses",
                GeneratorValue::as_value(&self.statuses),
            )?;

            receiver.map_field_entry(
                "search_metadata",
                GeneratorValue::as_value(&self.search_metadata),
            )?;

            receiver.type_tagged_map_end()
        }
    }

    pub struct Generator_Twitter<'a> {
        value: &'a Twitter,
        generator: GeneratorState_Twitter<'a>,
    }

    pub enum GeneratorState_Twitter<'a> {
        Begin,
        Field_statuses {
            generator:
                Option<<Vec<Status> as GeneratorValue>::Generator<'a>>,
        },
        Field_search_metadata {
            generator: Option<
                <SearchMetadata as GeneratorValue>::Generator<'a>,
            >,
        },
        End,
        Done,
    }

    impl<'a> Generator<'a> for Generator_Twitter<'a> {
        const MAY_YIELD: bool = true;

        fn resume<R: Receiver<'a>>(
            &mut self,
            receiver: &mut R,
        ) -> Result<GeneratorState> {
            match self.generator {
                GeneratorState_Twitter::Begin => {
                    receiver.type_tagged_map_begin(
                        tag::type_tag("Twitter"),
                        Some(2usize),
                    )?;
                    self.generator = GeneratorState_Twitter::Field_statuses { generator: None };
                    Ok(GeneratorState::Yield)
                }
                GeneratorState_Twitter::Field_statuses { ref mut generator } => {
                    if !<<Vec<Status> as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        receiver.map_field_entry("statuses", GeneratorValue::as_value(&self.value.statuses))?;
                    } else {
                        match generator {
                            None => {
                                receiver.map_field("statuses")?;
                                receiver.map_value_begin()?;
                                
                                let mut generator_slot = generator;
                                let mut generator = GeneratorValue::generator(&self.value.statuses);
                                
                                if let GeneratorState::Yield = generator.resume(receiver)? {
                                    *generator_slot = Some(generator);
                                    
                                    return Ok(GeneratorState::Yield);
                                }
                            }
                            Some(generator) => {
                                if let GeneratorState::Yield = generator.resume(receiver)? {
                                    return Ok(GeneratorState::Yield);
                                }
                            }
                        }
                        
                        receiver.map_value_end()?;
                    }
                    
                    self.generator = GeneratorState_Twitter::Field_search_metadata { generator: None };

                    Ok(GeneratorState::Yield)
                }
                GeneratorState_Twitter::Field_search_metadata { ref mut generator } => {
                    if !<<SearchMetadata as GeneratorValue>::Generator<'a>>::MAY_YIELD
                    {
                        receiver.map_field_entry(
                            "search_metadata",
                            GeneratorValue::as_value(&self.value.search_metadata),
                        )?;
                    } else {
                        match generator {
                            None => {
                                receiver.map_field("search_metadata")?;
                                receiver.map_value_begin()?;

                                let mut generator_slot = generator;
                                let mut generator = GeneratorValue::generator(&self.value.search_metadata);

                                if let GeneratorState::Yield = generator.resume(receiver)?
                                {
                                    *generator_slot = Some(generator);
                                    return Ok(GeneratorState::Yield);
                                }
                            }
                            Some(generator) => {
                                if let GeneratorState::Yield =
                                    generator.resume(receiver)?
                                {
                                    return Ok(GeneratorState::Yield);
                                }
                            }
                        }

                        receiver.map_value_end()?;
                    }

                    self.generator = GeneratorState_Twitter::End;
                    Ok(GeneratorState::Yield)
                }

                GeneratorState_Twitter::End => {
                    receiver.type_tagged_map_end()?;
                    self.generator = GeneratorState_Twitter::Done;
                    Ok(GeneratorState::Done)
                }

                GeneratorState_Twitter::Done => {
                    Ok(GeneratorState::Done)
                }
            }
        }
    }
};

```

```rust
// Using coroutines
const _: () = {
    impl CoroutineValue for Twitter {
        type State<'a> = CoroutineState_Twitter<'a>;
        type Coroutine<'a, R: Receiver<'a>> = Coroutine_Twitter_Begin;

        fn state<'a>(&'a self) -> Self::State<'a> {
            CoroutineState_Twitter {
                value: self,
                field: None,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.type_tagged_map_begin(tag::type_tag("Twitter"), Some(2usize))?;

            receiver.map_field_entry("statuses", CoroutineValue::as_value(&self.statuses))?;
            receiver.map_field_entry(
                "search_metadata",
                CoroutineValue::as_value(&self.search_metadata),
            )?;

            receiver.type_tagged_map_end()
        }
    }

    pub struct CoroutineState_Twitter<'a> {
        value: &'a Twitter,
        field: Option<CoroutineState_Twitter_Field<'a>>,
    }

    pub enum CoroutineState_Twitter_Field<'a> {
        Field_statuses(Slot<<Vec<Status> as CoroutineValue>::State<'a>>),
        Field_search_metadata(Slot<<SearchMetadata as CoroutineValue>::State<'a>>),
    }

    impl<'a> CoroutineState_Twitter<'a> {
        fn enter_field_statuses(
            self: std::pin::Pin<&mut Self>,
        ) -> std::pin::Pin<&mut Slot<<Vec<Status> as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = Some(CoroutineState_Twitter_Field::Field_statuses(Slot::new(
                CoroutineValue::state(&self_mut.value.statuses),
            )));

            if let Some(CoroutineState_Twitter_Field::Field_statuses(ref mut slot)) = self_mut.field {
                unsafe { std::pin::Pin::new_unchecked(slot) }
            } else {
                {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
        }

        fn enter_field_search_metadata(
            self: std::pin::Pin<&mut Self>,
        ) -> std::pin::Pin<&mut Slot<<SearchMetadata as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = Some(CoroutineState_Twitter_Field::Field_search_metadata(
                Slot::new(CoroutineValue::state(&self_mut.value.search_metadata)),
            ));

            if let Some(CoroutineState_Twitter_Field::Field_search_metadata(ref mut slot)) =
                self_mut.field
            {
                unsafe { std::pin::Pin::new_unchecked(slot) }
            } else {
                {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
        }

        fn exit_field_statuses(self: std::pin::Pin<&mut Self>) {
            unsafe { self.get_unchecked_mut() }.field = None;
        }

        fn exit_field_search_metadata(self: std::pin::Pin<&mut Self>) {
            unsafe { self.get_unchecked_mut() }.field = None;
        }
    }

    pub struct Coroutine_Twitter_Begin;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for Coroutine_Twitter_Begin {
        type State = CoroutineState_Twitter<'a>;

        const MAY_YIELD: bool = true;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            cx.receiver()
                .type_tagged_map_begin(tag::type_tag("Twitter"), Some(2usize))?;

            cx.yield_to::<Coroutine_Twitter_Field_statuses>()
        }
    }

    struct Coroutine_Twitter_Field_statuses;
    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for Coroutine_Twitter_Field_statuses {
        type State = CoroutineState_Twitter<'a>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            if !<<Vec<Status> as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                let (receiver, state) = cx.state();

                receiver
                    .map_field_entry("statuses", CoroutineValue::as_value(&state.value.statuses))?;

                cx.yield_to::<Coroutine_Twitter_Field_search_metadata>()
            } else {
                struct Exit;

                impl<'a, R: Receiver<'a>> Coroutine<'a, R> for Exit {
                    type State = CoroutineState_Twitter<'a>;

                    fn resume<'resume>(
                        mut cx: Context<'resume, R, Self>,
                    ) -> Result<Resume<'resume, Self>> {
                        let (receiver, state) = cx.state();

                        receiver.map_value_end()?;
                        state.exit_field_statuses();

                        cx.yield_to::<Coroutine_Twitter_Field_search_metadata>()
                    }
                }

                cx.receiver().map_field("statuses")?;

                cx.receiver().map_value_begin()?;

                cx.yield_into::<<Vec<Status> as CoroutineValue>::Coroutine<'a, R>, Exit, _>(|state| {
                    state.enter_field_statuses()
                })
            }
        }
    }

    struct Coroutine_Twitter_Field_search_metadata;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for Coroutine_Twitter_Field_search_metadata {
        type State = CoroutineState_Twitter<'a>;
        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            if !<<SearchMetadata as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                let (receiver, state) = cx.state();
                receiver.map_field_entry(
                    "search_metadata",
                    CoroutineValue::as_value(&state.value.search_metadata),
                )?;

                cx.yield_to::<Coroutine_Twitter_End>()
            } else {
                struct Exit;

                impl<'a, R: Receiver<'a>> Coroutine<'a, R> for Exit {
                    type State = CoroutineState_Twitter<'a>;

                    fn resume<'resume>(
                        mut cx: Context<'resume, R, Self>,
                    ) -> Result<Resume<'resume, Self>> {
                        let (receiver, state) = cx.state();

                        receiver.map_value_end()?;
                        state.exit_field_search_metadata();

                        cx.yield_to::<Coroutine_Twitter_End>()
                    }
                }

                cx.receiver().map_field("search_metadata")?;

                cx.receiver().map_value_begin()?;

                cx.yield_into::<<SearchMetadata as CoroutineValue>::Coroutine<'a, R>, Exit, _>(
                    |state| state.enter_field_search_metadata(),
                )
            }
        }
    }

    struct Coroutine_Twitter_End;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for Coroutine_Twitter_End {
        type State = CoroutineState_Twitter<'a>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            cx.receiver().type_tagged_map_end()?;

            cx.yield_return()
        }
    }
};
```
