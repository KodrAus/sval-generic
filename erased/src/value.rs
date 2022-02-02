use crate::receiver::Receiver;
use crate::value::private::DispatchValue;

mod private {
    use crate::receiver::Receiver;

    pub trait DispatchValue {
        fn dispatch_stream<'a>(&'a self, receiver: &mut dyn Receiver<'a>) -> sval::Result;
    }

    pub trait EraseValue {
        fn erase_value(&self) -> crate::private::Erased<&dyn DispatchValue>;
    }
}

pub trait Value: private::EraseValue {}

impl<T: sval::SourceValue> Value for T {}

impl<T: sval::SourceValue> private::EraseValue for T {
    fn erase_value(&self) -> crate::private::Erased<&dyn DispatchValue> {
        crate::private::Erased(self)
    }
}

impl<T: sval::SourceValue> private::DispatchValue for T {
    fn dispatch_stream<'a>(&'a self, receiver: &mut dyn Receiver<'a>) -> sval::Result {
        self.stream(receiver)
    }
}

macro_rules! impl_value {
    ($($impl:tt)*) => {
        $($impl)* {
            fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
                self.erase_value().0.dispatch_stream(&mut receiver)
            }
        }
    }
}

impl_value!(impl<'d> sval::SourceValue for dyn Value + 'd);
impl_value!(impl<'d> sval::SourceValue for dyn Value + Send + 'd);
impl_value!(impl<'d> sval::SourceValue for dyn Value + Send + Sync + 'd);
