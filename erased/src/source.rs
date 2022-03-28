use crate::source::private::DispatchSource;
use crate::Receiver;

mod private {
    use crate::receiver::Receiver;

    pub trait DispatchSource<'a> {
        fn dispatch_stream_resume<'b>(
            &mut self,
            receiver: &mut dyn Receiver<'b>,
        ) -> sval::Result<sval::Resume>
        where
            'a: 'b;

        fn dispatch_stream_to_end<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result
        where
            'a: 'b;

        fn dispatch_maybe_dynamic(&self) -> Option<bool>;
    }

    pub trait EraseSource<'a> {
        fn erase_source_ref(&self) -> crate::private::Erased<&dyn DispatchSource<'a>>;
        fn erase_source(&mut self) -> crate::private::Erased<&mut dyn DispatchSource<'a>>;
    }
}

pub trait Source<'a>: private::EraseSource<'a> {}

impl<'a, S: sval::Source<'a>> Source<'a> for S {}

impl<'a, S: sval::Source<'a>> private::EraseSource<'a> for S {
    fn erase_source_ref(&self) -> crate::private::Erased<&dyn DispatchSource<'a>> {
        crate::private::Erased(self)
    }

    fn erase_source(&mut self) -> crate::private::Erased<&mut dyn private::DispatchSource<'a>> {
        crate::private::Erased(self)
    }
}

impl<'a, S: sval::Source<'a>> private::DispatchSource<'a> for S {
    fn dispatch_stream_resume<'b>(
        &mut self,
        receiver: &mut dyn Receiver<'b>,
    ) -> sval::Result<sval::Resume>
    where
        'a: 'b,
    {
        self.stream_resume(receiver)
    }

    fn dispatch_stream_to_end<'b>(&mut self, receiver: &mut dyn Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        self.stream_to_end(receiver)
    }

    fn dispatch_maybe_dynamic(&self) -> Option<bool> {
        self.maybe_dynamic()
    }
}

macro_rules! impl_source {
    ($($impl:tt)*) => {
        $($impl)* {
            fn stream_resume<'b, R: sval::Receiver<'b>>(&mut self, mut receiver: R) -> sval::Result<sval::Resume>
            where
                'a: 'b,
            {
                self.erase_source().0.dispatch_stream_resume(&mut receiver)
            }

            fn stream_to_end<'b, R: sval::Receiver<'b>>(&mut self, mut receiver: R) -> sval::Result
            where
                'a: 'b,
            {
                self.erase_source().0.dispatch_stream_to_end(&mut receiver)
            }

            fn maybe_dynamic(&self) -> Option<bool> {
                self.erase_source_ref().0.dispatch_maybe_dynamic()
            }
        }
    }
}

impl_source!(impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + 'd);
impl_source!(impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + Send + 'd);
impl_source!(impl<'a, 'd> sval::Source<'a> for dyn Source<'a> + Send + Sync + 'd);
