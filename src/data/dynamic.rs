use crate::{std::mem, Receiver, Result, Value};

pub fn dynamic<T: Value + ?Sized>(value: &T) -> &Dynamic<T> {
    Dynamic::new(value)
}

#[repr(transparent)]
pub struct Dynamic<T: ?Sized>(T);

impl<T: ?Sized> Dynamic<T> {
    pub fn new<'a>(value: &'a T) -> &'a Self {
        unsafe { mem::transmute::<&'a T, &'a Dynamic<T>>(value) }
    }
}

impl<T: Value + ?Sized> Value for Dynamic<T> {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        receiver.dynamic_begin()?;
        self.0.stream(&mut receiver)?;
        receiver.dynamic_end()
    }

    fn is_dynamic(&self) -> bool {
        true
    }

    fn to_bool(&self) -> Option<bool> {
        self.0.to_bool()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_text(&self) -> Option<&str> {
        self.0.to_text()
    }

    fn to_binary(&self) -> Option<&[u8]> {
        self.0.to_binary()
    }
}
