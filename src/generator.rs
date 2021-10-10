use std::{marker::PhantomData, mem, ops::ControlFlow};

use crate::{Error, Receiver, Result, Value};

pub trait GeneratorValue {
    #[doc(hidden)]
    type Generator<'a, R: Receiver<'a>>: GeneratorStep;

    fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self>;

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.stream_begin(receiver).stream()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        self.stream_begin(receiver).into_iter().collect()
    }

    fn as_value(&self) -> AsValue<&Self> {
        AsValue(self)
    }

    fn as_value_iter(&self) -> AsValueIter<&Self> {
        AsValueIter(self)
    }
}

impl<'b, T: GeneratorValue + ?Sized> GeneratorValue for &'b T {
    type Generator<'a, R: Receiver<'a>> = ByRef<T::Generator<'a, R>>;

    fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
        Generator(ByRef((**self).stream_begin(receiver).0))
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream_iter(receiver)
    }

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream(receiver)
    }
}

#[doc(hidden)]
pub struct ByRef<G>(G);

impl<G: GeneratorStep> GeneratorStep for ByRef<G> {
    type Machine = G::Machine;
    type Next = G::Next;

    fn machine(self) -> Self::Machine {
        self.0.machine()
    }

    fn stream(self) -> Result {
        self.0.stream()
    }

    fn stream_continue(self) -> Result<Self::Next> {
        self.0.stream_continue()
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

pub struct Generator<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized>(V::Generator<'a, R>);

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Generator<'a, R, V> {
    pub fn stream(self) -> Result {
        self.0.stream()
    }

    pub fn into_iter(self) -> IntoIter<'a, R, V> {
        IntoIter(self.0.machine())
    }
}

pub struct IntoIter<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized>(
    <V::Generator<'a, R> as GeneratorStep>::Machine,
);

impl<'a, R: Receiver<'a>, V: GeneratorValue + ?Sized> Iterator for IntoIter<'a, R, V> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.stream_continue() {
            Ok(ControlFlow::Continue(())) => Some(Ok(())),
            Ok(ControlFlow::Break(())) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[doc(hidden)]
pub trait GeneratorMachine {
    fn stream_continue(&mut self) -> Result<ControlFlow<(), ()>>;
    fn is_done(&self) -> bool;
}

#[doc(hidden)]
pub trait GeneratorStep {
    type Machine: GeneratorMachine;
    type Next: GeneratorStep;

    fn machine(self) -> Self::Machine;
    fn stream(self) -> Result;
    fn stream_continue(self) -> Result<Self::Next>;
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
        type Generator<'a, R: Receiver<'a>> = MyTypeGeneratorStep0<'a, R>;

        fn stream_begin<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Generator<'a, R, Self> {
            Generator(MyTypeGeneratorStep0 {
                value: self,
                receiver,
            })
        }
    }

    pub enum MyTypeGeneratorMachine<'a, R> {
        Step0(MyTypeGeneratorStep0<'a, R>),
        Step1(MyTypeGeneratorStep1<'a, R>),
        Done,
    }

    impl<'a, R: Receiver<'a>> GeneratorMachine for MyTypeGeneratorMachine<'a, R> {
        fn stream_continue(&mut self) -> Result<ControlFlow<(), ()>> {
            match mem::replace(self, MyTypeGeneratorMachine::Done) {
                MyTypeGeneratorMachine::Step0(step) => {
                    *self = MyTypeGeneratorMachine::Step1(step.stream_continue()?);
                    Ok(ControlFlow::Continue(()))
                }
                // Terminal step
                MyTypeGeneratorMachine::Step1(step) => {
                    step.stream_continue()?;
                    Ok(ControlFlow::Break(()))
                }
                // Sentinel terminal step
                MyTypeGeneratorMachine::Done => Ok(ControlFlow::Break(())),
            }
        }

        fn is_done(&self) -> bool {
            matches!(self, MyTypeGeneratorMachine::Done)
        }
    }

    pub struct MyTypeGeneratorStep0<'a, R> {
        value: &'a MyType,
        receiver: R,
    }

    impl<'a, R: Receiver<'a>> GeneratorStep for MyTypeGeneratorStep0<'a, R> {
        type Machine = MyTypeGeneratorMachine<'a, R>;
        type Next = MyTypeGeneratorStep1<'a, R>;

        fn machine(self) -> Self::Machine {
            MyTypeGeneratorMachine::Step0(self)
        }

        #[inline]
        fn stream(mut self) -> Result {
            self.receiver.map_field_entry("a", &self.value.a)?;
            self.receiver.map_field_entry("b", &self.value.b)?;

            Ok(())
        }

        fn stream_continue(mut self) -> Result<Self::Next> {
            self.receiver.map_field_entry("a", &self.value.a)?;

            Ok(MyTypeGeneratorStep1 {
                value: self.value,
                receiver: self.receiver,
            })
        }
    }

    pub struct MyTypeGeneratorStep1<'a, R> {
        value: &'a MyType,
        receiver: R,
    }

    impl<'a, R: Receiver<'a>> GeneratorStep for MyTypeGeneratorStep1<'a, R> {
        type Machine = MyTypeGeneratorMachine<'a, R>;
        type Next = MyTypeGeneratorDone<'a, R>;

        fn machine(self) -> Self::Machine {
            MyTypeGeneratorMachine::Step1(self)
        }

        #[inline]
        fn stream(mut self) -> Result {
            self.receiver.map_field_entry("b", &self.value.b)?;

            Ok(())
        }

        fn stream_continue(mut self) -> Result<Self::Next> {
            self.receiver.map_field_entry("b", &self.value.b)?;

            Ok(MyTypeGeneratorDone(Default::default()))
        }
    }

    pub struct MyTypeGeneratorDone<'a, R>(PhantomData<(&'a MyType, R)>);

    impl<'a, R: Receiver<'a>> GeneratorStep for MyTypeGeneratorDone<'a, R> {
        type Machine = MyTypeGeneratorMachine<'a, R>;
        type Next = MyTypeGeneratorDone<'a, R>;

        fn machine(self) -> Self::Machine {
            MyTypeGeneratorMachine::Done
        }

        #[inline]
        fn stream(self) -> Result {
            Ok(())
        }

        fn stream_continue(mut self) -> Result<Self::Next> {
            Err(Error)
        }
    }
};
