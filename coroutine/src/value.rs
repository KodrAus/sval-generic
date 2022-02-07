use crate::{co::Resume, Receiver};

use std::pin::Pin;

use sval::{Result, Value};

pub fn source<'a>(v: &'a impl CoroutineValue) -> impl sval::Source<'a> {
    struct Source<S>(S);

    impl<'a, V: CoroutineValue> sval::Source<'a> for Source<&'a V> {
        fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<sval::source::Next>
        where
            'a: 'b,
        {
            self.stream_all(receiver).map(|_| sval::source::Next::Done)
        }

        fn stream_all<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
        where
            'a: 'b,
        {
            let mut co = crate::co::Coroutine::new(self.0.state());

            let source = crate::co::RefMutSource::<R, V::Coroutine<'b, R>>::new(receiver, unsafe {
                Pin::new_unchecked(&mut co)
            });

            source.into_iter().collect()
        }
    }

    Source(v)
}

pub trait CoroutineValue: Value {
    type State<'a>
    where
        Self: 'a;
    type Coroutine<'a, R: Receiver<'a>>: Resume<'a, R, State = Self::State<'a>>
    where
        Self: 'a;

    fn state<'a>(&'a self) -> Self::State<'a>;
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
}
