use crate::{Receiver, Result};

use super::GeneratorValue;

pub enum GeneratorState {
    Yield,
    Done,
}

pub trait Coroutine<'a> {
    const MAY_YIELD: bool = true;

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
