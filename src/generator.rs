use std::{marker::PhantomData, mem};

use crate::{tag::type_tag, Error, Receiver, Result, Value};

pub trait GeneratorValue {
    type Context: ?Sized;

    #[doc(hidden)]
    type Generator<'a>: GeneratorMachine<'a, Context = Self::Context>;

    #[doc(hidden)]
    fn generator<'a>(&'a self) -> Self::Generator<'a>;

    fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self>;

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

impl<'b, T: GeneratorValue + ?Sized> GeneratorValue for &'b T {
    type Context = T::Context;
    type Generator<'a> = ByRef<T::Generator<'a>>;

    fn generator<'a>(&'a self) -> Self::Generator<'a> {
        ByRef((**self).generator())
    }

    fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
        let Generator {
            generator,
            context,
            receiver,
        } = (**self).stream_begin(receiver);

        Generator {
            generator: ByRef(generator),
            context,
            receiver,
        }
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream_iter(receiver)
    }

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream(receiver)
    }
}

pub struct AsValue<V>(V);

impl<V: GeneratorValue> Value for AsValue<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.0.stream(receiver)
    }
}

pub struct AsValueIter<V>(V);

impl<V: GeneratorValue> Value for AsValueIter<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.0.stream_iter(receiver)
    }
}

#[doc(hidden)]
pub enum GeneratorFlow {
    Yield,
    Done,
}

#[doc(hidden)]
pub trait GeneratorMachine<'a> {
    type Context: ?Sized;

    const MAY_YIELD: bool;

    fn stream_continue<R: Receiver<'a>>(
        &mut self,
        cx: &'a Self::Context,
        receiver: &mut R,
    ) -> Result<GeneratorFlow>;
}

pub struct Generator<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> {
    generator: V::Generator<'a>,
    context: &'a V::Context,
    receiver: R,
}

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Generator<'a, R, V> {
    pub fn into_iter(self) -> IntoIter<'a, R, V> {
        IntoIter {
            generator: self.generator,
            context: self.context,
            receiver: self.receiver,
        }
    }
}

pub struct IntoIter<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> {
    generator: V::Generator<'a>,
    context: &'a V::Context,
    receiver: R,
}

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Iterator for IntoIter<'a, R, V> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .generator
            .stream_continue(self.context, &mut self.receiver)
        {
            Ok(GeneratorFlow::Yield) => Some(Ok(())),
            Ok(GeneratorFlow::Done) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[doc(hidden)]
pub struct ByRef<G>(G);

impl<'a, G: GeneratorMachine<'a>> GeneratorMachine<'a> for ByRef<G> {
    type Context = G::Context;

    const MAY_YIELD: bool = G::MAY_YIELD;

    fn stream_continue<R: Receiver<'a>>(
        &mut self,
        cx: &'a Self::Context,
        receiver: &mut R,
    ) -> Result<GeneratorFlow> {
        self.0.stream_continue(cx, receiver)
    }
}

pub struct MyType {
    pub a: i32,
    pub b: String,
}

impl Value for MyType {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.map_field_entry("a", &self.a)?;
        receiver.map_field_entry("b", &self.b)?;

        Ok(())
    }
}

#[allow(dead_code, unused_mut)]
const IMPL_MYTYPE_GENERATOR: () = {
    impl GeneratorValue for MyType {
        type Context = Self;
        type Generator<'a> = MyTypeGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            MyTypeGenerator::Begin
        }

        fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
            Generator {
                generator: self.generator(),
                context: self,
                receiver,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.map_field_entry("a", &self.a)?;
            receiver.map_field_entry("b", &self.b)?;

            Ok(())
        }
    }

    pub enum MyTypeGenerator<'a> {
        Begin,
        Fielda {
            generator: Option<<i32 as GeneratorValue>::Generator<'a>>,
        },
        Fieldb {
            generator: Option<<String as GeneratorValue>::Generator<'a>>,
        },
        End,
        Done,
    }

    impl<'a> GeneratorMachine<'a> for MyTypeGenerator<'a> {
        type Context = MyType;
        const MAY_YIELD: bool = true;

        fn stream_continue<R: Receiver<'a>>(
            &mut self,
            cx: &'a Self::Context,
            receiver: &mut R,
        ) -> Result<GeneratorFlow> {
            match self {
                // map_begin
                MyTypeGenerator::Begin => {
                    receiver.type_tagged_map_begin(type_tag("MyType"), Some(2))?;

                    *self = MyTypeGenerator::Fielda { generator: None };

                    Ok(GeneratorFlow::Yield)
                }

                // self.a
                MyTypeGenerator::Fielda { ref mut generator } => {
                    // primitive generator
                    if !<<i32 as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        receiver.map_field_entry("a", &cx.a)?;
                    }
                    // complex generator
                    else {
                        match generator {
                            // key
                            None => {
                                receiver.map_field("a")?;
                                receiver.map_value_begin()?;

                                *generator = Some(cx.a.generator());

                                return Ok(GeneratorFlow::Yield);
                            }
                            // value
                            Some(generator) => match generator.stream_continue(&cx.a, receiver)? {
                                GeneratorFlow::Done => {
                                    receiver.map_value_end()?;
                                }
                                GeneratorFlow::Yield => return Ok(GeneratorFlow::Yield),
                            },
                        }
                    }

                    *self = MyTypeGenerator::Fieldb { generator: None };

                    Ok(GeneratorFlow::Yield)
                }

                // self.b
                MyTypeGenerator::Fieldb { ref mut generator } => {
                    // primitive generator
                    if !<<String as GeneratorValue>::Generator<'a>>::MAY_YIELD {
                        receiver.map_field_entry("b", &cx.b)?;
                    }
                    // complex generator
                    else {
                        match generator {
                            // key
                            None => {
                                receiver.map_field("b")?;
                                receiver.map_value_begin()?;

                                *generator = Some(cx.b.generator());

                                return Ok(GeneratorFlow::Yield);
                            }
                            // value
                            Some(generator) => match generator.stream_continue(&cx.b, receiver)? {
                                GeneratorFlow::Done => {
                                    receiver.map_value_end()?;
                                }
                                GeneratorFlow::Yield => return Ok(GeneratorFlow::Yield),
                            },
                        }
                    }

                    *self = MyTypeGenerator::End;

                    Ok(GeneratorFlow::Yield)
                }

                // end
                MyTypeGenerator::End => {
                    receiver.type_tagged_map_end()?;

                    *self = MyTypeGenerator::Done;

                    Ok(GeneratorFlow::Done)
                }

                // done
                MyTypeGenerator::Done => Ok(GeneratorFlow::Done),
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const IMPL_I32_GENERATOR: () = {
    impl GeneratorValue for i32 {
        type Context = Self;
        type Generator<'a> = I32Generator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            I32Generator(Default::default())
        }

        fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
            Generator {
                generator: self.generator(),
                context: self,
                receiver,
            }
        }
    }

    pub struct I32Generator<'a>(PhantomData<&'a i32>);

    impl<'a> GeneratorMachine<'a> for I32Generator<'a> {
        type Context = i32;

        const MAY_YIELD: bool = false;

        #[inline]
        fn stream_continue<R: Receiver<'a>>(
            &mut self,
            cx: &'a Self::Context,
            receiver: &mut R,
        ) -> Result<GeneratorFlow> {
            receiver.i32(*cx)?;

            Ok(GeneratorFlow::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const IMPL_STR_GENERATOR: () = {
    impl GeneratorValue for str {
        type Context = Self;
        type Generator<'a> = StrGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            StrGenerator(Default::default())
        }

        fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
            Generator {
                generator: self.generator(),
                context: self,
                receiver,
            }
        }
    }

    impl GeneratorValue for String {
        type Context = str;
        type Generator<'a> = StrGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            StrGenerator(Default::default())
        }

        fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
            Generator {
                generator: self.generator(),
                context: self,
                receiver,
            }
        }
    }

    pub struct StrGenerator<'a>(PhantomData<&'a str>);

    impl<'a> GeneratorMachine<'a> for StrGenerator<'a> {
        type Context = str;

        const MAY_YIELD: bool = false;

        #[inline]
        fn stream_continue<R: Receiver<'a>>(
            &mut self,
            cx: &'a Self::Context,
            receiver: &mut R,
        ) -> Result<GeneratorFlow> {
            receiver.str(cx)?;

            Ok(GeneratorFlow::Done)
        }
    }
};
