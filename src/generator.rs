use std::{marker::PhantomData, mem};

use crate::{Error, Receiver, Result, Value};

pub trait GeneratorValue {
    #[doc(hidden)]
    type Generator<'a>: GeneratorStep<'a>;

    #[doc(hidden)]
    fn generator<'a>(&'a self) -> Self::Generator<'a>;

    fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
        Generator {
            generator: self.generator(),
            receiver,
        }
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

impl<'b, T: GeneratorValue + ?Sized> GeneratorValue for &'b T {
    type Generator<'a> = ByRef<T::Generator<'a>>;

    fn generator<'a>(&'a self) -> Self::Generator<'a> {
        ByRef((**self).generator())
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
    fn stream_continue<R: Receiver<'a>>(&mut self, receiver: R) -> Result<GeneratorFlow>;
    fn is_done(&self) -> bool;
}

#[doc(hidden)]
pub trait GeneratorStep<'a> {
    type Machine: GeneratorMachine<'a>;

    const IS_TERMINAL: bool;

    fn machine(self) -> Self::Machine;
    fn stream_continue<R: Receiver<'a>>(&mut self, receiver: R) -> Result<GeneratorFlow>;
}

pub struct Generator<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> {
    generator: V::Generator<'a>,
    receiver: R,
}

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Generator<'a, R, V> {
    pub fn into_iter(self) -> IntoIter<'a, R, V> {
        IntoIter {
            machine: self.generator.machine(),
            receiver: self.receiver,
        }
    }
}

pub struct IntoIter<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> {
    machine: <V::Generator<'a> as GeneratorStep<'a>>::Machine,
    receiver: R,
}

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Iterator for IntoIter<'a, R, V> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        match self.machine.stream_continue(&mut self.receiver) {
            Ok(GeneratorFlow::Yield) => Some(Ok(())),
            Ok(GeneratorFlow::Done) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[doc(hidden)]
pub struct ByRef<G>(G);

impl<'a, G: GeneratorStep<'a>> GeneratorStep<'a> for ByRef<G> {
    type Machine = G::Machine;

    const IS_TERMINAL: bool = G::IS_TERMINAL;

    fn machine(self) -> Self::Machine {
        self.0.machine()
    }

    fn stream_continue<R: Receiver<'a>>(&mut self, receiver: R) -> Result<GeneratorFlow> {
        self.0.stream_continue(receiver)
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
        type Generator<'a> = MyTypeGeneratorStep0<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            MyTypeGeneratorStep0 {
                value: self,
                machine: None,
            }
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.map_field_entry("a", &self.a)?;
            receiver.map_field_entry("b", &self.b)?;

            Ok(())
        }
    }

    pub enum MyTypeGeneratorMachine<'a> {
        Step0(MyTypeGeneratorStep0<'a>),
        Step1(MyTypeGeneratorStep1<'a>),
        Done,
    }

    impl<'a> GeneratorMachine<'a> for MyTypeGeneratorMachine<'a> {
        fn stream_continue<R: Receiver<'a>>(&mut self, receiver: R) -> Result<GeneratorFlow> {
            match self {
                // self.a
                MyTypeGeneratorMachine::Step0(step) => match step.stream_continue(receiver)? {
                    GeneratorFlow::Done => {
                        *self = MyTypeGeneratorMachine::Step1(MyTypeGeneratorStep1 {
                            value: step.value,
                            machine: None,
                        });

                        Ok(GeneratorFlow::Yield)
                    }
                    GeneratorFlow::Yield => Ok(GeneratorFlow::Yield),
                },

                // self.b
                MyTypeGeneratorMachine::Step1(step) => match step.stream_continue(receiver)? {
                    GeneratorFlow::Done => {
                        *self = MyTypeGeneratorMachine::Done;

                        Ok(GeneratorFlow::Done)
                    }
                    GeneratorFlow::Yield => Ok(GeneratorFlow::Yield),
                },

                // done
                MyTypeGeneratorMachine::Done => Ok(GeneratorFlow::Done),
            }
        }

        fn is_done(&self) -> bool {
            matches!(self, MyTypeGeneratorMachine::Done)
        }
    }

    pub struct MyTypeGeneratorStep0<'a> {
        value: &'a MyType,
        machine: Option<<<i32 as GeneratorValue>::Generator<'a> as GeneratorStep<'a>>::Machine>,
    }

    impl<'a> GeneratorStep<'a> for MyTypeGeneratorStep0<'a> {
        type Machine = MyTypeGeneratorMachine<'a>;

        const IS_TERMINAL: bool = false;

        fn machine(self) -> Self::Machine {
            MyTypeGeneratorMachine::Step0(self)
        }

        #[inline]
        fn stream_continue<R: Receiver<'a>>(&mut self, mut receiver: R) -> Result<GeneratorFlow> {
            // primitive generator
            if <<i32 as GeneratorValue>::Generator<'a> as GeneratorStep<'a>>::IS_TERMINAL {
                receiver.map_field_entry("a", &self.value.a)?;

                Ok(GeneratorFlow::Done)
            }
            // complex generator
            else {
                match self.machine {
                    // key
                    None => {
                        receiver.map_field("a")?;

                        self.machine = Some(self.value.a.generator().machine());

                        receiver.map_value_begin()?;
                        Ok(GeneratorFlow::Yield)
                    }
                    // value
                    Some(ref mut machine) => match machine.stream_continue(&mut receiver)? {
                        GeneratorFlow::Done => {
                            receiver.map_value_end()?;
                            Ok(GeneratorFlow::Done)
                        }
                        GeneratorFlow::Yield => Ok(GeneratorFlow::Yield),
                    },
                }
            }
        }
    }

    pub struct MyTypeGeneratorStep1<'a> {
        value: &'a MyType,
        machine: Option<<<String as GeneratorValue>::Generator<'a> as GeneratorStep<'a>>::Machine>,
    }

    impl<'a> GeneratorStep<'a> for MyTypeGeneratorStep1<'a> {
        type Machine = MyTypeGeneratorMachine<'a>;

        const IS_TERMINAL: bool = true;

        fn machine(self) -> Self::Machine {
            MyTypeGeneratorMachine::Step1(self)
        }

        #[inline]
        fn stream_continue<R: Receiver<'a>>(&mut self, mut receiver: R) -> Result<GeneratorFlow> {
            // primitive generator
            if <<String as GeneratorValue>::Generator<'a> as GeneratorStep<'a>>::IS_TERMINAL {
                receiver.map_field_entry("b", &self.value.b)?;

                Ok(GeneratorFlow::Done)
            }
            // complex generator
            else {
                match self.machine {
                    // key
                    None => {
                        receiver.map_field("b")?;

                        self.machine = Some(self.value.b.generator().machine());

                        receiver.map_value_begin()?;
                        Ok(GeneratorFlow::Yield)
                    }
                    // value
                    Some(ref mut machine) => match machine.stream_continue(&mut receiver)? {
                        GeneratorFlow::Done => {
                            receiver.map_value_end()?;
                            Ok(GeneratorFlow::Done)
                        }
                        GeneratorFlow::Yield => Ok(GeneratorFlow::Yield),
                    },
                }
            }
        }
    }
};

#[allow(dead_code, unused_mut)]
const IMPL_I32_GENERATOR: () = {
    impl GeneratorValue for i32 {
        type Generator<'a> = I32Generator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            I32Generator { value: Some(self) }
        }
    }

    impl<'a> GeneratorMachine<'a> for I32GeneratorMachine<'a> {
        fn stream_continue<R: Receiver<'a>>(&mut self, mut receiver: R) -> Result<GeneratorFlow> {
            self.generator.stream_continue(receiver)
        }

        fn is_done(&self) -> bool {
            self.generator.value.is_none()
        }
    }

    pub struct I32Generator<'a> {
        value: Option<&'a i32>,
    }

    pub struct I32GeneratorMachine<'a> {
        generator: I32Generator<'a>,
    }

    impl<'a> GeneratorStep<'a> for I32Generator<'a> {
        type Machine = I32GeneratorMachine<'a>;

        const IS_TERMINAL: bool = true;

        fn machine(self) -> Self::Machine {
            I32GeneratorMachine { generator: self }
        }

        #[inline]
        fn stream_continue<R: Receiver<'a>>(&mut self, mut receiver: R) -> Result<GeneratorFlow> {
            if let Some(value) = self.value.take() {
                receiver.i32(*value)?;
            }

            Ok(GeneratorFlow::Done)
        }
    }
};

#[allow(dead_code, unused_mut)]
const IMPL_STR_GENERATOR: () = {
    impl GeneratorValue for str {
        type Generator<'a> = StrGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            StrGenerator { value: Some(self) }
        }
    }

    impl GeneratorValue for String {
        type Generator<'a> = StrGenerator<'a>;

        fn generator<'a>(&'a self) -> Self::Generator<'a> {
            StrGenerator { value: Some(self) }
        }
    }

    impl<'a> GeneratorMachine<'a> for StrGeneratorMachine<'a> {
        fn stream_continue<R: Receiver<'a>>(&mut self, mut receiver: R) -> Result<GeneratorFlow> {
            self.generator.stream_continue(receiver)
        }

        fn is_done(&self) -> bool {
            self.generator.value.is_none()
        }
    }

    pub struct StrGenerator<'a> {
        value: Option<&'a str>,
    }

    pub struct StrGeneratorMachine<'a> {
        generator: StrGenerator<'a>,
    }

    impl<'a> GeneratorStep<'a> for StrGenerator<'a> {
        type Machine = StrGeneratorMachine<'a>;

        const IS_TERMINAL: bool = true;

        fn machine(self) -> Self::Machine {
            StrGeneratorMachine { generator: self }
        }

        #[inline]
        fn stream_continue<R: Receiver<'a>>(&mut self, mut receiver: R) -> Result<GeneratorFlow> {
            if let Some(value) = self.value.take() {
                receiver.str(value)?;
            }

            Ok(GeneratorFlow::Done)
        }
    }
};
