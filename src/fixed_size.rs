pub fn fixed_size<T>(value: T) -> FixedSize<T> {
    FixedSize::new(value)
}

#[derive(Clone, Copy)]
pub struct FixedSize<T>(pub(crate) T);

impl<T> FixedSize<T> {
    pub fn new(value: T) -> Self {
        FixedSize(value)
    }

    pub fn by_ref(&self) -> FixedSize<&T> {
        FixedSize(&self.0)
    }

    pub fn by_mut(&mut self) -> FixedSize<&mut T> {
        FixedSize(&mut self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
