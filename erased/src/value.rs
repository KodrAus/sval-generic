use crate::receiver::Receiver;
use crate::value::private::DispatchValue;

mod private {
    use crate::receiver::Receiver;

    pub trait DispatchValue {
        fn dispatch_stream<'a>(&'a self, receiver: &mut dyn Receiver<'a>) -> sval::Result;
        fn dispatch_is_dynamic(&self) -> bool;
        fn dispatch_to_bool(&self) -> Option<bool>;
        fn dispatch_to_f32(&self) -> Option<f32>;
        fn dispatch_to_f64(&self) -> Option<f64>;
        fn dispatch_to_i8(&self) -> Option<i8>;
        fn dispatch_to_i16(&self) -> Option<i16>;
        fn dispatch_to_i32(&self) -> Option<i32>;
        fn dispatch_to_i64(&self) -> Option<i64>;
        fn dispatch_to_i128(&self) -> Option<i128>;
        fn dispatch_to_u8(&self) -> Option<u8>;
        fn dispatch_to_u16(&self) -> Option<u16>;
        fn dispatch_to_u32(&self) -> Option<u32>;
        fn dispatch_to_u64(&self) -> Option<u64>;
        fn dispatch_to_u128(&self) -> Option<u128>;
        fn dispatch_to_text(&self) -> Option<&str>;
        fn dispatch_to_binary(&self) -> Option<&[u8]>;
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

    fn dispatch_is_dynamic(&self) -> bool {
        self.is_dynamic()
    }

    fn dispatch_to_bool(&self) -> Option<bool> {
        self.to_bool()
    }

    fn dispatch_to_f32(&self) -> Option<f32> {
        self.to_f32()
    }

    fn dispatch_to_f64(&self) -> Option<f64> {
        self.to_f64()
    }

    fn dispatch_to_i8(&self) -> Option<i8> {
        self.to_i8()
    }

    fn dispatch_to_i16(&self) -> Option<i16> {
        self.to_i16()
    }

    fn dispatch_to_i32(&self) -> Option<i32> {
        self.to_i32()
    }

    fn dispatch_to_i64(&self) -> Option<i64> {
        self.to_i64()
    }

    fn dispatch_to_i128(&self) -> Option<i128> {
        self.to_i128()
    }

    fn dispatch_to_u8(&self) -> Option<u8> {
        self.to_u8()
    }

    fn dispatch_to_u16(&self) -> Option<u16> {
        self.to_u16()
    }

    fn dispatch_to_u32(&self) -> Option<u32> {
        self.to_u32()
    }

    fn dispatch_to_u64(&self) -> Option<u64> {
        self.to_u64()
    }

    fn dispatch_to_u128(&self) -> Option<u128> {
        self.to_u128()
    }

    fn dispatch_to_text(&self) -> Option<&str> {
        self.to_text()
    }

    fn dispatch_to_binary(&self) -> Option<&[u8]> {
        self.to_binary()
    }
}

macro_rules! impl_value {
    ($($impl:tt)*) => {
        $($impl)* {
            fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
                self.erase_value().0.dispatch_stream(&mut receiver)
            }

            fn is_dynamic(&self) -> bool {
                self.erase_value().0.dispatch_is_dynamic()
            }

            fn to_bool(&self) -> Option<bool> {
                self.erase_value().0.dispatch_to_bool()
            }

            fn to_f32(&self) -> Option<f32> {
                self.erase_value().0.dispatch_to_f32()
            }

            fn to_f64(&self) -> Option<f64> {
                self.erase_value().0.dispatch_to_f64()
            }

            fn to_i8(&self) -> Option<i8> {
                self.erase_value().0.dispatch_to_i8()
            }

            fn to_i16(&self) -> Option<i16> {
                self.erase_value().0.dispatch_to_i16()
            }

            fn to_i32(&self) -> Option<i32> {
                self.erase_value().0.dispatch_to_i32()
            }

            fn to_i64(&self) -> Option<i64> {
                self.erase_value().0.dispatch_to_i64()
            }

            fn to_i128(&self) -> Option<i128> {
                self.erase_value().0.dispatch_to_i128()
            }

            fn to_u8(&self) -> Option<u8> {
                self.erase_value().0.dispatch_to_u8()
            }

            fn to_u16(&self) -> Option<u16> {
                self.erase_value().0.dispatch_to_u16()
            }

            fn to_u32(&self) -> Option<u32> {
                self.erase_value().0.dispatch_to_u32()
            }

            fn to_u64(&self) -> Option<u64> {
                self.erase_value().0.dispatch_to_u64()
            }

            fn to_u128(&self) -> Option<u128> {
                self.erase_value().0.dispatch_to_u128()
            }

            fn to_text(&self) -> Option<&str> {
                self.erase_value().0.dispatch_to_text()
            }

            fn to_binary(&self) -> Option<&[u8]> {
                self.erase_value().0.dispatch_to_binary()
            }
        }
    }
}

impl_value!(impl<'d> sval::Value for dyn Value + 'd);
impl_value!(impl<'d> sval::Value for dyn Value + Send + 'd);
impl_value!(impl<'d> sval::Value for dyn Value + Send + Sync + 'd);
