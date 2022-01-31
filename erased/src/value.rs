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

impl<T: sval::Value> Value for T {}

impl<T: sval::Value> private::EraseValue for T {
    fn erase_value(&self) -> crate::private::Erased<&dyn DispatchValue> {
        crate::private::Erased(self)
    }
}

impl<T: sval::Value> private::DispatchValue for T {
    fn dispatch_stream<'a>(&'a self, receiver: &mut dyn Receiver<'a>) -> sval::Result {
        self.stream(receiver)
    }
}

impl<'d> sval::Value for dyn Value + 'd {
    fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
        self.erase_value().0.dispatch_stream(&mut receiver)
    }
}
