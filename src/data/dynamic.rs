use crate::{source, std::mem, Receiver, Result, Source, Value};

#[inline]
pub fn dynamic<T>(value: T) -> Dynamic<T> {
    Dynamic::new(value)
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Dynamic<T>(T);

impl<T> Dynamic<T> {
    pub fn new(value: T) -> Self {
        Dynamic(value)
    }

    pub fn new_ref(value: &T) -> &Self {
        unsafe { mem::transmute::<&T, &Dynamic<T>>(value) }
    }

    pub fn new_mut(value: &mut T) -> &mut Self {
        unsafe { mem::transmute::<&mut T, &mut Dynamic<T>>(value) }
    }

    pub fn by_ref(&self) -> Dynamic<&T> {
        Dynamic(&self.0)
    }

    pub fn by_mut(&mut self) -> Dynamic<&mut T> {
        Dynamic(&mut self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Value> Value for Dynamic<T> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
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

impl<'a, T: Source<'a>> Source<'a> for Dynamic<T> {
    fn stream_resume<'b, S: Receiver<'b>>(&mut self, receiver: S) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, S: Receiver<'b>>(&mut self, mut receiver: S) -> Result
    where
        'a: 'b,
    {
        receiver.dynamic_begin()?;
        self.0.stream_to_end(&mut receiver)?;
        receiver.dynamic_end()
    }
}
