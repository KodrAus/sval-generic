use std::pin::Pin;

mod internal;
pub use self::internal::*;

use crate::{tag, Receiver, Result, Source, Value};

pub trait CoroutineValue {
    #[doc(hidden)]
    type State<'a>
    where
        Self: 'a;

    #[doc(hidden)]
    type Coroutine<'a, R: Receiver<'a>>: Coroutine<'a, R, State = Self::State<'a>>
    where
        Self: 'a;

    #[doc(hidden)]
    fn state<'a>(&'a self) -> Self::State<'a>;

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        let mut state = Slot::new(self.state());

        Driver::<R, Self::Coroutine<'a, R>>::new(receiver, unsafe {
            Pin::new_unchecked(&mut state)
        })
        .into_iter()
        .collect()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        let mut state = Slot::new(self.state());

        Driver::<R, Self::Coroutine<'a, R>>::new(receiver, unsafe {
            Pin::new_unchecked(&mut state)
        })
        .into_iter()
        .collect()
    }

    // These just exist so we can bench the generated code
    #[inline]
    fn as_value(&self) -> AsValue<&Self> {
        AsValue(self)
    }

    #[inline]
    fn as_value_iter(&self) -> AsValueIter<&Self> {
        AsValueIter(self)
    }
}

pub struct AsValue<V>(V);

impl<V: CoroutineValue> Value for AsValue<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.0.stream(receiver)
    }
}

impl<'a, V: CoroutineValue + ?Sized> Source<'a> for AsValue<&'a V> {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        self.0.stream(receiver)
    }
}

pub struct AsValueIter<V>(V);

impl<V: CoroutineValue> Value for AsValueIter<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.0.stream_iter(receiver)
    }
}

impl<'b, T: CoroutineValue + ?Sized> CoroutineValue for &'b T {
    type State<'a>
    where
        T: 'a,
        'b: 'a,
    = T::State<'a>;

    type Coroutine<'a, R: Receiver<'a>>
    where
        T: 'a,
        'b: 'a,
    = T::Coroutine<'a, R>;

    #[inline]
    fn state<'a>(&'a self) -> Self::State<'a> {
        (**self).state()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream_iter(receiver)
    }

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream(receiver)
    }
}

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for () {
        type State<'a> = ();

        type Coroutine<'a, R: Receiver<'a>> = UnitCoroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            ()
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.none()
        }
    }

    pub struct UnitCoroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for UnitCoroutine {
        type State = ();

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            cx.receiver().none()?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for i32 {
        type State<'a> = i32;

        type Coroutine<'a, R: Receiver<'a>> = I32Coroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.i32(*self)
        }
    }

    pub struct I32Coroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for I32Coroutine {
        type State = i32;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.i32(*state)?;

            cx.yield_return()
        }
    }
};

pub struct MyType {
    pub a: i32,
    pub b: i32,
}

#[allow(dead_code, unused_mut, non_camel_case_types)]
const _: () = {
    impl CoroutineValue for MyType {
        type State<'a> = MyTypeCoroutineState<'a>;

        type Coroutine<'a, R: Receiver<'a>> = MyTypeCoroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            MyTypeCoroutineState {
                value: self,
                field: None,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.type_tagged_map_begin(tag::type_tag("MyType"), Some(2))?;

            receiver.map_field_entry("a", &self.a)?;
            receiver.map_field_entry("b", &self.b)?;

            receiver.type_tagged_map_end()
        }
    }

    pub struct MyTypeCoroutineState<'a> {
        value: &'a MyType,
        field: Option<MyTypeCoroutineField<'a>>,
    }

    enum MyTypeCoroutineField<'a> {
        Field_a(Slot<<i32 as CoroutineValue>::State<'a>>),
        Field_b(Slot<<i32 as CoroutineValue>::State<'a>>),
    }

    impl<'a> MyTypeCoroutineState<'a> {
        #[inline]
        fn enter_field_a(
            self: Pin<&mut Self>,
        ) -> Pin<&mut Slot<<i32 as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = Some(MyTypeCoroutineField::Field_a(Slot::new(
                self_mut.value.a.state(),
            )));

            if let Some(MyTypeCoroutineField::Field_a(ref mut slot)) = self_mut.field {
                unsafe { Pin::new_unchecked(slot) }
            } else {
                unreachable!()
            }
        }

        #[inline]
        fn exit_field_a(self: Pin<&mut Self>) {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = None;
        }

        #[inline]
        fn enter_field_b(
            self: Pin<&mut Self>,
        ) -> Pin<&mut Slot<<i32 as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = Some(MyTypeCoroutineField::Field_b(Slot::new(
                self_mut.value.b.state(),
            )));

            if let Some(MyTypeCoroutineField::Field_b(ref mut slot)) = self_mut.field {
                unsafe { Pin::new_unchecked(slot) }
            } else {
                unreachable!()
            }
        }

        #[inline]
        fn exit_field_b(self: Pin<&mut Self>) {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = None;
        }
    }

    pub struct MyTypeCoroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for MyTypeCoroutine {
        type State = MyTypeCoroutineState<'a>;

        const MAY_YIELD: bool = true;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            cx.receiver()
                .type_tagged_map_begin(tag::type_tag("MyType"), Some(2))?;

            cx.yield_to::<MyTypeCoroutineField_a>()
        }
    }

    struct MyTypeCoroutineField_a;
    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for MyTypeCoroutineField_a {
        type State = MyTypeCoroutineState<'a>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            // Fast path: we don't need to yield into `a`
            if !<<i32 as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                let (receiver, state) = cx.state();

                receiver.map_field_entry("a", &state.value.a)?;

                cx.yield_to::<MyTypeCoroutineField_b>()
            }
            // Slow path: we do need to yield into `a`
            else {
                struct MyTypeCoroutineEndField_a;
                impl<'a, R: Receiver<'a>> Coroutine<'a, R> for MyTypeCoroutineEndField_a {
                    type State = MyTypeCoroutineState<'a>;

                    fn resume<'resume>(
                        mut cx: Context<'resume, R, Self>,
                    ) -> Result<Resume<'resume, Self>> {
                        let (receiver, state) = cx.state();

                        receiver.map_value_end()?;

                        state.exit_field_a();

                        cx.yield_to::<MyTypeCoroutineField_b>()
                    }
                }

                cx.receiver().map_field("a")?;
                cx.receiver().map_value_begin()?;

                cx.yield_into::<<i32 as CoroutineValue>::Coroutine<'a, R>, MyTypeCoroutineEndField_a>(|state| {
                    state.enter_field_a()
                })
            }
        }
    }

    struct MyTypeCoroutineField_b;
    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for MyTypeCoroutineField_b {
        type State = MyTypeCoroutineState<'a>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            // Fast path: we don't need to yield into `b`
            if !<<i32 as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                let (receiver, state) = cx.state();

                receiver.map_field_entry("b", &state.value.b)?;

                cx.yield_to::<MyTypeCoroutineEnd>()
            }
            // Slow path: we do need to yield into `b`
            else {
                struct MyTypeCoroutineEndField_b;
                impl<'a, R: Receiver<'a>> Coroutine<'a, R> for MyTypeCoroutineEndField_b {
                    type State = MyTypeCoroutineState<'a>;

                    fn resume<'resume>(
                        mut cx: Context<'resume, R, Self>,
                    ) -> Result<Resume<'resume, Self>> {
                        let (receiver, state) = cx.state();

                        receiver.map_value_end()?;

                        state.exit_field_b();

                        cx.yield_to::<MyTypeCoroutineEnd>()
                    }
                }

                cx.receiver().map_field("b")?;
                cx.receiver().map_value_begin()?;

                cx.yield_into::<<i32 as CoroutineValue>::Coroutine<'a, R>, MyTypeCoroutineEndField_b>(|state| {
                    state.enter_field_b()
                })
            }
        }
    }

    struct MyTypeCoroutineEnd;
    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for MyTypeCoroutineEnd {
        type State = MyTypeCoroutineState<'a>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            cx.receiver().type_tagged_map_end()?;

            cx.yield_return()
        }
    }
};
