use std::pin::Pin;

mod internal;
pub use self::internal::{Context, Coroutine, RefMut, Resume, Slot};

use crate::{Receiver, Result, Source, Value};

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

        RefMut::<R, Self::Coroutine<'a, R>>::new(receiver, unsafe {
            Pin::new_unchecked(&mut state)
        })
        .into_iter()
        .collect()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        let mut state = Slot::new(self.state());

        RefMut::<R, Self::Coroutine<'a, R>>::new(receiver, unsafe {
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
    impl CoroutineValue for bool {
        type State<'a> = bool;

        type Coroutine<'a, R: Receiver<'a>> = BoolCoroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.bool(*self)
        }
    }

    pub struct BoolCoroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for BoolCoroutine {
        type State = bool;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.bool(*state)?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for u8 {
        type State<'a> = u8;

        type Coroutine<'a, R: Receiver<'a>> = U8Coroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u8(*self)
        }
    }

    pub struct U8Coroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for U8Coroutine {
        type State = u8;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.u8(*state)?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for u16 {
        type State<'a> = u16;

        type Coroutine<'a, R: Receiver<'a>> = U16Coroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u16(*self)
        }
    }

    pub struct U16Coroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for U16Coroutine {
        type State = u16;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.u16(*state)?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for u32 {
        type State<'a> = u32;

        type Coroutine<'a, R: Receiver<'a>> = U32Coroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u32(*self)
        }
    }

    pub struct U32Coroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for U32Coroutine {
        type State = u32;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.u32(*state)?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for u64 {
        type State<'a> = u64;

        type Coroutine<'a, R: Receiver<'a>> = U64Coroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u64(*self)
        }
    }

    pub struct U64Coroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for U64Coroutine {
        type State = u64;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.u64(*state)?;

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

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for f32 {
        type State<'a> = f32;

        type Coroutine<'a, R: Receiver<'a>> = F32Coroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            *self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.f32(*self)
        }
    }

    pub struct F32Coroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for F32Coroutine {
        type State = f32;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.f32(*state)?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for str {
        type State<'a> = &'a str;

        type Coroutine<'a, R: Receiver<'a>> = StrCoroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.str(self)
        }
    }

    impl CoroutineValue for String {
        type State<'a> = &'a str;

        type Coroutine<'a, R: Receiver<'a>> = StrCoroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            self
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.str(self)
        }
    }

    pub struct StrCoroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for StrCoroutine {
        type State = &'a str;

        const MAY_YIELD: bool = false;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.str(*state)?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: CoroutineValue + ?Sized> CoroutineValue for Box<T> {
        type State<'a>
        where
            Self: 'a,
        = BoxCoroutineState<'a, T>;

        type Coroutine<'a, R: Receiver<'a>>
        where
            Self: 'a,
        = BoxCoroutine<'a, T>;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            BoxCoroutineState {
                value: self,
                slot: None,
            }
        }

        #[inline]
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            (**self).stream(receiver)
        }

        #[inline]
        fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
            (**self).stream_iter(receiver)
        }
    }

    pub struct BoxCoroutineState<'a, T: CoroutineValue + ?Sized> {
        value: &'a T,
        slot: Option<Box<Slot<<T as CoroutineValue>::State<'a>>>>,
    }

    impl<'a, T: CoroutineValue + ?Sized> BoxCoroutineState<'a, T> {
        #[inline]
        fn enter_value(self: Pin<&mut Self>) -> Pin<&mut Slot<<T as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.slot = Some(Box::new(Slot::new(self_mut.value.state())));

            match self_mut.slot {
                Some(ref mut slot) => unsafe { Pin::new_unchecked(slot) },
                None => unreachable!(),
            }
        }
    }

    pub struct BoxCoroutine<'a, T: ?Sized>(PhantomData<&'a T>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue + 'a + ?Sized> Coroutine<'a, R>
        for BoxCoroutine<'a, T>
    {
        type State = BoxCoroutineState<'a, T>;

        const MAY_YIELD: bool =
            <<T as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, mut state) = cx.state();

            if !<<T as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                receiver.source(state.value.as_value())?;

                cx.yield_return()
            } else {
                cx.yield_into_return::<<T as CoroutineValue>::Coroutine<'a, R>>(|state| {
                    state.enter_value()
                })
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: CoroutineValue> CoroutineValue for Option<T> {
        type State<'a>
        where
            Self: 'a,
        = OptionCoroutineState<'a, T>;

        type Coroutine<'a, R: Receiver<'a>>
        where
            Self: 'a,
        = OptionCoroutine<'a, T>;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            OptionCoroutineState {
                value: self.as_ref(),
                slot: None,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            if let Some(value) = self {
                receiver.source(value.as_value())
            } else {
                receiver.none()
            }
        }
    }

    pub struct OptionCoroutineState<'a, T: CoroutineValue> {
        value: Option<&'a T>,
        slot: Option<Slot<<T as CoroutineValue>::State<'a>>>,
    }

    impl<'a, T: CoroutineValue> OptionCoroutineState<'a, T> {
        #[inline]
        fn enter_value(self: Pin<&mut Self>) -> Pin<&mut Slot<<T as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.slot = Some(Slot::new(self_mut.value.unwrap().state()));

            match self_mut.slot {
                Some(ref mut slot) => unsafe { Pin::new_unchecked(slot) },
                None => unreachable!(),
            }
        }
    }

    pub struct OptionCoroutine<'a, T>(PhantomData<&'a [T]>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue + 'a> Coroutine<'a, R> for OptionCoroutine<'a, T> {
        type State = OptionCoroutineState<'a, T>;

        const MAY_YIELD: bool =
            <<T as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, mut state) = cx.state();

            match state.value {
                Some(value) => {
                    if !<<T as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                        receiver.source(value.as_value())?;

                        cx.yield_return()
                    } else {
                        cx.yield_into_return::<<T as CoroutineValue>::Coroutine<'a, R>>(|state| {
                            state.enter_value()
                        })
                    }
                }
                None => {
                    receiver.none()?;

                    cx.yield_return()
                }
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: CoroutineValue> CoroutineValue for Vec<T> {
        type State<'a>
        where
            Self: 'a,
        = ArrayCoroutineState<'a, T>;

        type Coroutine<'a, R: Receiver<'a>>
        where
            Self: 'a,
        = ArrayCoroutineBegin<'a, T>;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            ArrayCoroutineState {
                value: self,
                index: 0,
                element: None,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.seq_begin(Some(self.len()))?;

            for elem in self {
                receiver.seq_elem(elem.as_value())?;
            }

            receiver.seq_end()
        }
    }

    pub struct ArrayCoroutineState<'a, T: CoroutineValue> {
        value: &'a [T],
        index: usize,
        element: Option<Slot<<T as CoroutineValue>::State<'a>>>,
    }

    impl<'a, T: CoroutineValue> ArrayCoroutineState<'a, T> {
        #[inline]
        fn last_element(&self) -> bool {
            self.index == self.value.len() - 1
        }

        #[inline]
        fn next_element(self: Pin<&mut Self>) -> usize {
            let self_mut = unsafe { self.get_unchecked_mut() };

            let i = self_mut.index;
            self_mut.index += 1;
            i
        }

        #[inline]
        fn enter_elem(
            mut self: Pin<&mut Self>,
        ) -> Pin<&mut Slot<<T as CoroutineValue>::State<'a>>> {
            let i = self.as_mut().next_element();

            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.element = Some(Slot::new(self_mut.value[i].state()));

            match self_mut.element {
                Some(ref mut elem) => unsafe { Pin::new_unchecked(elem) },
                None => unreachable!(),
            }
        }

        #[inline]
        fn exit_elem(self: Pin<&mut Self>) -> bool {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.element.take().is_some()
        }
    }

    pub struct ArrayCoroutineBegin<'a, T>(PhantomData<&'a [T]>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue> Coroutine<'a, R> for ArrayCoroutineBegin<'a, T> {
        type State = ArrayCoroutineState<'a, T>;

        const MAY_YIELD: bool = true;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            receiver.seq_begin(Some(state.value.len()))?;

            if state.value.len() == 0 {
                cx.yield_resume::<ArrayCoroutineEnd<T>>()
            } else {
                cx.yield_resume::<ArrayCoroutineElement<T>>()
            }
        }
    }

    struct ArrayCoroutineElement<'a, T>(PhantomData<&'a [T]>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue> Coroutine<'a, R> for ArrayCoroutineElement<'a, T> {
        type State = ArrayCoroutineState<'a, T>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, mut state) = cx.state();

            if !<<T as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                if state.last_element() {
                    let i = state.as_mut().next_element();
                    receiver.seq_elem(state.value[i].as_value())?;

                    cx.yield_resume::<ArrayCoroutineEnd<T>>()
                } else {
                    let i = state.as_mut().next_element();
                    receiver.seq_elem(state.value[i].as_value())?;

                    cx.yield_resume_self()
                }
            } else {
                if state.as_mut().exit_elem() {
                    receiver.seq_elem_end()?;
                }

                receiver.seq_elem_begin()?;

                if state.as_mut().last_element() {
                    cx.yield_into_resume::<<T as CoroutineValue>::Coroutine<'a, R>, ArrayCoroutineEnd<T>>(|state| {
                        state.enter_elem()
                    })
                } else {
                    cx.yield_into_resume_self::<<T as CoroutineValue>::Coroutine<'a, R>>(|state| {
                        state.enter_elem()
                    })
                }
            }
        }
    }

    struct ArrayCoroutineEnd<'a, T>(PhantomData<&'a [T]>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue> Coroutine<'a, R> for ArrayCoroutineEnd<'a, T> {
        type State = ArrayCoroutineState<'a, T>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            if state.exit_elem() {
                receiver.seq_elem_end()?;
            }

            receiver.seq_end()?;

            cx.yield_return()
        }
    }
};

#[allow(dead_code, unused_mut, non_camel_case_types)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: CoroutineValue, U: CoroutineValue> CoroutineValue for (T, U) {
        type State<'a>
        where
            Self: 'a,
        = TupleCoroutineState<'a, T, U>;

        type Coroutine<'a, R: Receiver<'a>>
        where
            Self: 'a,
        = TupleCoroutineBegin<'a, T, U>;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            TupleCoroutineState {
                value: self,
                field: None,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.seq_begin(Some(2))?;

            receiver.seq_elem(self.0.as_value())?;
            receiver.seq_elem(self.1.as_value())?;

            receiver.seq_end()
        }
    }

    pub struct TupleCoroutineState<'a, T: CoroutineValue, U: CoroutineValue> {
        value: &'a (T, U),
        field: Option<TupleCoroutineStateField<'a, T, U>>,
    }

    enum TupleCoroutineStateField<'a, T: CoroutineValue + 'a, U: CoroutineValue + 'a> {
        Field_0(Slot<<T as CoroutineValue>::State<'a>>),
        Field_1(Slot<<U as CoroutineValue>::State<'a>>),
    }

    impl<'a, T: CoroutineValue, U: CoroutineValue> TupleCoroutineState<'a, T, U> {
        #[inline]
        fn enter_field_0(self: Pin<&mut Self>) -> Pin<&mut Slot<<T as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = Some(TupleCoroutineStateField::Field_0(Slot::new(
                self_mut.value.0.state(),
            )));

            match self_mut.field {
                Some(TupleCoroutineStateField::Field_0(ref mut slot)) => unsafe {
                    Pin::new_unchecked(slot)
                },
                _ => unreachable!(),
            }
        }

        #[inline]
        fn enter_field_1(self: Pin<&mut Self>) -> Pin<&mut Slot<<U as CoroutineValue>::State<'a>>> {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field = Some(TupleCoroutineStateField::Field_1(Slot::new(
                self_mut.value.1.state(),
            )));

            match self_mut.field {
                Some(TupleCoroutineStateField::Field_1(ref mut slot)) => unsafe {
                    Pin::new_unchecked(slot)
                },
                _ => unreachable!(),
            }
        }

        #[inline]
        fn exit_field(self: Pin<&mut Self>) -> bool {
            let self_mut = unsafe { self.get_unchecked_mut() };

            self_mut.field.take().is_some()
        }
    }

    pub struct TupleCoroutineBegin<'a, T, U>(PhantomData<&'a (T, U)>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue, U: CoroutineValue> Coroutine<'a, R>
        for TupleCoroutineBegin<'a, T, U>
    {
        type State = TupleCoroutineState<'a, T, U>;

        const MAY_YIELD: bool = true;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            cx.receiver().seq_begin(Some(2))?;

            cx.yield_resume::<TupleCoroutineField_0<T, U>>()
        }
    }

    struct TupleCoroutineField_0<'a, T, U>(PhantomData<&'a (T, U)>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue, U: CoroutineValue> Coroutine<'a, R>
        for TupleCoroutineField_0<'a, T, U>
    {
        type State = TupleCoroutineState<'a, T, U>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, mut state) = cx.state();

            if !<<T as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                receiver.seq_elem(state.value.0.as_value())?;

                cx.yield_resume::<TupleCoroutineField_1<T, U>>()
            } else {
                receiver.seq_elem_begin()?;

                cx.yield_into_resume::<<T as CoroutineValue>::Coroutine<'a, R>, TupleCoroutineField_1<T, U>>(
                    |state| state.enter_field_0(),
                )
            }
        }
    }

    struct TupleCoroutineField_1<'a, T, U>(PhantomData<&'a (T, U)>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue, U: CoroutineValue> Coroutine<'a, R>
        for TupleCoroutineField_1<'a, T, U>
    {
        type State = TupleCoroutineState<'a, T, U>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, mut state) = cx.state();

            if state.as_mut().exit_field() {
                receiver.seq_elem_end()?;
            }

            if !<<U as CoroutineValue>::Coroutine<'a, R> as Coroutine<'a, R>>::MAY_YIELD {
                receiver.seq_elem(state.value.1.as_value())?;

                cx.yield_resume::<TupleCoroutineEnd<T, U>>()
            } else {
                receiver.seq_elem_begin()?;

                cx.yield_into_resume::<<U as CoroutineValue>::Coroutine<'a, R>, TupleCoroutineEnd<T, U>>(
                    |state| state.enter_field_1(),
                )
            }
        }
    }

    struct TupleCoroutineEnd<'a, T, U>(PhantomData<&'a (T, U)>);
    impl<'a, R: Receiver<'a>, T: CoroutineValue, U: CoroutineValue> Coroutine<'a, R>
        for TupleCoroutineEnd<'a, T, U>
    {
        type State = TupleCoroutineState<'a, T, U>;

        fn resume<'resume>(mut cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>> {
            let (receiver, state) = cx.state();

            if state.exit_field() {
                receiver.seq_elem_end()?;
            }

            receiver.seq_end()?;

            cx.yield_return()
        }
    }
};
