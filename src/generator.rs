use crate::{Error, Receiver, Result, Source, Value};

pub trait GeneratorValue {
    #[doc(hidden)]
    type Generator<'a>: GeneratorImpl<'a>
    where
        Self: 'a;

    fn generator<'a>(&'a self) -> Self::Generator<'a>;

    fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
        Generator::begin(receiver, self)
    }

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.stream_begin(receiver).into_iter().collect()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.stream_begin(receiver).into_iter().collect()
    }

    // These just exist so we can bench the generated code
    fn as_value(&self) -> AsValue<&Self> {
        AsValue(self)
    }

    fn as_value_iter(&self) -> AsValueIter<&Self> {
        AsValueIter(self)
    }
}

pub struct AsValue<V>(V);

impl<V: GeneratorValue> Value for AsValue<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.0.stream(receiver)
    }
}

impl<'a, V: GeneratorValue + ?Sized> Source<'a> for AsValue<&'a V> {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        self.0.stream(receiver)
    }
}

pub struct AsValueIter<V>(V);

impl<V: GeneratorValue> Value for AsValueIter<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.0.stream_iter(receiver)
    }
}

impl<'b, T: GeneratorValue + ?Sized> GeneratorValue for &'b T {
    type Generator<'a>
    where
        T: 'a,
        'b: 'a,
    = T::Generator<'a>;

    fn generator<'a>(&'a self) -> Self::Generator<'a> {
        (**self).generator()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream_iter(receiver)
    }

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream(receiver)
    }
}

#[doc(hidden)]
pub enum GeneratorState {
    Yield,
    Done,
}

#[doc(hidden)]
pub trait GeneratorImpl<'a> {
    // TODO: We need `MAY_YIELD` hints on `Receiver` too
    const MAY_YIELD: bool = true;

    // TODO: Self: `Pin<&mut Self>`?
    fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState>;
}

pub struct Generator<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized + 'a> {
    generator: V::Generator<'a>,
    receiver: R,
}

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Generator<'a, R, V> {
    pub fn begin(receiver: R, value: &'a V) -> Self {
        Generator {
            generator: value.generator(),
            receiver,
        }
    }

    pub fn into_iter(self) -> IntoIter<'a, R, V> {
        IntoIter {
            generator: self.generator,
            receiver: self.receiver,
        }
    }
}

pub struct IntoIter<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized + 'a> {
    generator: V::Generator<'a>,
    receiver: R,
}

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Iterator for IntoIter<'a, R, V> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generator.resume(&mut self.receiver) {
            Ok(GeneratorState::Yield) => Some(Ok(())),
            Ok(GeneratorState::Done) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for () {
        type Generator<'a> = UnitGenerator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            UnitGenerator
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.none()
        }
    }

    pub struct UnitGenerator;

    impl<'a> GeneratorImpl<'a> for UnitGenerator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.none()?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for u8 {
        type Generator<'a> = U8Generator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            U8Generator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u8(*self)
        }
    }

    pub struct U8Generator(u8);

    impl<'a> GeneratorImpl<'a> for U8Generator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.u8(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for u16 {
        type Generator<'a> = U16Generator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            U16Generator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u16(*self)
        }
    }

    pub struct U16Generator(u16);

    impl<'a> GeneratorImpl<'a> for U16Generator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.u16(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for u32 {
        type Generator<'a> = U32Generator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            U32Generator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u32(*self)
        }
    }

    pub struct U32Generator(u32);

    impl<'a> GeneratorImpl<'a> for U32Generator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.u32(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for u64 {
        type Generator<'a> = U64Generator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            U64Generator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.u64(*self)
        }
    }

    pub struct U64Generator(u64);

    impl<'a> GeneratorImpl<'a> for U64Generator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.u64(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for i32 {
        type Generator<'a> = I32Generator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            I32Generator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.i32(*self)
        }
    }

    pub struct I32Generator(i32);

    impl<'a> GeneratorImpl<'a> for I32Generator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.i32(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for f32 {
        type Generator<'a> = F32Generator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            F32Generator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.f32(*self)
        }
    }

    pub struct F32Generator(f32);

    impl<'a> GeneratorImpl<'a> for F32Generator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.f32(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for bool {
        type Generator<'a> = BoolGenerator;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            BoolGenerator(*self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.bool(*self)
        }
    }

    pub struct BoolGenerator(bool);

    impl<'a> GeneratorImpl<'a> for BoolGenerator {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.bool(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl GeneratorValue for str {
        type Generator<'a> = StrGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            StrGenerator(self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.str(self)
        }
    }

    impl GeneratorValue for String {
        type Generator<'a> = StrGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            StrGenerator(self)
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.str(self)
        }
    }

    pub struct StrGenerator<'a>(&'a str);

    impl<'a> GeneratorImpl<'a> for StrGenerator<'a> {
        const MAY_YIELD: bool = false;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            receiver.str(self.0)?;

            Ok(GeneratorState::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    impl<T: GeneratorValue + ?Sized> GeneratorValue for Box<T> {
        type Generator<'a>
        where
            Self: 'a,
        = BoxGenerator<'a, T>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            BoxGenerator::Value { value: self }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
            (**self).stream(receiver)
        }
    }

    pub enum BoxGenerator<'a, T: GeneratorValue + ?Sized + 'a> {
        Value {
            value: &'a T,
        },
        Resume {
            generator: Box<<T as GeneratorValue>::Generator<'a>>,
        },
    };

    impl<'a, T: GeneratorValue + ?Sized + 'a> GeneratorImpl<'a> for BoxGenerator<'a, T> {
        const MAY_YIELD: bool = <T::Generator<'a> as GeneratorImpl<'a>>::MAY_YIELD;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            match self {
                BoxGenerator::Value { value } => {
                    if !<<T as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        value.stream(receiver)?;

                        Ok(GeneratorState::Done)
                    } else {
                        let mut generator = value.generator();

                        match generator.resume(receiver)? {
                            GeneratorState::Done => Ok(GeneratorState::Done),
                            GeneratorState::Yield => {
                                *self = BoxGenerator::Resume {
                                    generator: Box::new(generator),
                                };
                                Ok(GeneratorState::Yield)
                            }
                        }
                    }
                }
                BoxGenerator::Resume { ref mut generator } => generator.resume(receiver),
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: GeneratorValue> GeneratorValue for Option<T> {
        type Generator<'a>
        where
            Self: 'a,
        = OptionGenerator<'a, T>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            OptionGenerator::Value {
                value: self.as_ref(),
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            match self {
                Some(v) => receiver.source(v.as_value()),
                None => receiver.none(),
            }
        }
    }

    pub enum OptionGenerator<'a, T: GeneratorValue + 'a> {
        Value {
            value: Option<&'a T>,
        },
        Resume {
            generator: <T as GeneratorValue>::Generator<'a>,
        },
    }

    impl<'a, T: GeneratorValue + 'a> GeneratorImpl<'a> for OptionGenerator<'a, T> {
        const MAY_YIELD: bool = <T::Generator<'a>>::MAY_YIELD;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            match self {
                OptionGenerator::Value { value: Some(value) } => {
                    if !<<T as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        value.stream(receiver)?;

                        Ok(GeneratorState::Done)
                    } else {
                        let mut generator = value.generator();

                        match generator.resume(receiver)? {
                            GeneratorState::Done => Ok(GeneratorState::Done),
                            GeneratorState::Yield => {
                                *self = OptionGenerator::Resume { generator };
                                Ok(GeneratorState::Yield)
                            }
                        }
                    }
                }
                OptionGenerator::Value { value: None } => {
                    receiver.none()?;

                    Ok(GeneratorState::Done)
                }
                OptionGenerator::Resume { ref mut generator } => generator.resume(receiver),
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: GeneratorValue, U: GeneratorValue> GeneratorValue for (T, U) {
        type Generator<'a>
        where
            Self: 'a,
        = TupleGenerator<'a, T, U>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            TupleGenerator {
                value: (&self.0, &self.1),
                generator: TupleGeneratorState::Begin,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.seq_begin(Some(2))?;

            receiver.seq_elem(self.0.as_value())?;
            receiver.seq_elem(self.1.as_value())?;

            receiver.seq_end()
        }
    }

    pub struct TupleGenerator<'a, T: GeneratorValue + 'a, U: GeneratorValue + 'a> {
        value: (&'a T, &'a U),
        generator: TupleGeneratorState<'a, T, U>,
    }

    pub enum TupleGeneratorState<'a, T: GeneratorValue + 'a, U: GeneratorValue + 'a> {
        Begin,
        Field_0 {
            generator: Option<<T as GeneratorValue>::Generator<'a>>,
        },
        Field_1 {
            generator: Option<<U as GeneratorValue>::Generator<'a>>,
        },
        End,
        Done,
    }

    impl<'a, T: GeneratorValue + 'a, U: GeneratorValue + 'a> GeneratorImpl<'a>
        for TupleGenerator<'a, T, U>
    {
        const MAY_YIELD: bool = true;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            match self.generator {
                TupleGeneratorState::Begin => {
                    receiver.seq_begin(Some(2))?;

                    self.generator = TupleGeneratorState::Field_0 { generator: None };

                    Ok(GeneratorState::Yield)
                }

                TupleGeneratorState::Field_0 { ref mut generator } => {
                    if !<<T as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        receiver.seq_elem(self.value.0.as_value())?;
                    } else {
                        match generator {
                            None => {
                                receiver.seq_elem_begin()?;

                                let mut generator_slot = generator;
                                let mut generator = self.value.0.generator();

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

                        receiver.seq_elem_end()?;
                    }

                    self.generator = TupleGeneratorState::Field_1 { generator: None };

                    Ok(GeneratorState::Yield)
                }

                TupleGeneratorState::Field_1 { ref mut generator } => {
                    if !<<U as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        receiver.seq_elem(self.value.1.as_value())?;
                    } else {
                        match generator {
                            None => {
                                receiver.seq_elem_begin()?;

                                let mut generator_slot = generator;
                                let mut generator = self.value.1.generator();

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

                        receiver.seq_elem_end()?;
                    }

                    self.generator = TupleGeneratorState::End;

                    Ok(GeneratorState::Yield)
                }

                TupleGeneratorState::End => {
                    receiver.seq_end()?;

                    self.generator = TupleGeneratorState::Done;

                    Ok(GeneratorState::Done)
                }

                TupleGeneratorState::Done => Ok(GeneratorState::Done),
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const _: () = {
    use std::marker::PhantomData;

    impl<T: GeneratorValue> GeneratorValue for Vec<T> {
        type Generator<'a>
        where
            Self: 'a,
        = ArrayGenerator<'a, T>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            ArrayGenerator {
                value: self,
                generator: ArrayGeneratorState::Begin,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.seq_begin(Some(2))?;

            for elem in self {
                receiver.seq_elem(elem.as_value())?;
            }

            receiver.seq_end()
        }
    }

    pub struct ArrayGenerator<'a, T: GeneratorValue + 'a> {
        value: &'a [T],
        generator: ArrayGeneratorState<'a, T>,
    }

    pub enum ArrayGeneratorState<'a, T: GeneratorValue + 'a> {
        Begin,
        Elem {
            index: usize,
            generator: Option<<T as GeneratorValue>::Generator<'a>>,
        },
        End,
        Done,
    }

    impl<'a, T: GeneratorValue + 'a> GeneratorImpl<'a> for ArrayGenerator<'a, T> {
        const MAY_YIELD: bool = true;

        fn resume<R: Receiver<'a>>(&mut self, receiver: &mut R) -> Result<GeneratorState> {
            match self.generator {
                ArrayGeneratorState::Begin => {
                    receiver.seq_begin(Some(self.value.len()))?;

                    if self.value.len() > 0 {
                        self.generator = ArrayGeneratorState::Elem {
                            index: 0,
                            generator: None,
                        };
                    } else {
                        self.generator = ArrayGeneratorState::End;
                    }

                    Ok(GeneratorState::Yield)
                }

                ArrayGeneratorState::Elem {
                    ref mut index,
                    ref mut generator,
                } => {
                    if !<<T as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        receiver.seq_elem(self.value[*index].as_value())?;
                    } else {
                        match generator {
                            None => {
                                receiver.seq_elem_begin()?;

                                let mut generator_slot = &mut *generator;
                                let mut generator = self.value[*index].generator();

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

                        receiver.seq_elem_end()?;

                        *generator = None;
                    }

                    *index += 1;

                    if *index >= self.value.len() {
                        self.generator = ArrayGeneratorState::End;
                    }

                    Ok(GeneratorState::Yield)
                }

                ArrayGeneratorState::End => {
                    receiver.seq_end()?;

                    self.generator = ArrayGeneratorState::Done;

                    Ok(GeneratorState::Done)
                }

                ArrayGeneratorState::Done => Ok(GeneratorState::Done),
            }
        }
    }
};
