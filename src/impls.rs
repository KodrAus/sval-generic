use crate::value::{self, Value};

impl Value for () {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.none()
    }
}

impl Value for bool {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.bool(*self)
    }
}

impl Value for u8 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.u64(*self as u64)
    }
}

impl Value for i8 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i64(*self as i64)
    }
}

impl Value for u16 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.u64(*self as u64)
    }
}

impl Value for i16 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i64(*self as i64)
    }
}

impl Value for u32 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.u64(*self as u64)
    }
}

impl Value for i32 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i64(*self as i64)
    }
}

impl Value for u64 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.u64(*self)
    }
}

impl Value for u128 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.u128(*self)
    }
}

impl Value for i128 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i128(*self)
    }
}

impl Value for f32 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.f64(*self as f64)
    }
}

impl Value for f64 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.f64(*self)
    }
}

impl Value for str {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.str(self)
    }

    fn to_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl Value for String {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.str(&**self)
    }

    fn to_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<T> Value for Option<T>
where
    T: Value,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        match self {
            Some(v) => v.stream(stream),
            None => stream.none(),
        }
    }
}

impl<T> Value for [T]
where
    T: Value,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_elem(elem)?;
        }

        stream.seq_end()
    }
}

impl<T> Value for Vec<T>
where
    T: Value,
{
    fn stream<'a, S>(&'a self, stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        (&**self).stream(stream)
    }
}

impl<T, U> Value for (T, U)
where
    T: Value,
    U: Value,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.seq_begin(Some(2))?;
        stream.seq_elem(&self.0)?;
        stream.seq_elem(&self.1)?;
        stream.seq_end()
    }
}

impl<T: ?Sized> Value for Box<T>
where
    T: Value,
{
    fn stream<'a, S>(&'a self, stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        (**self).stream(stream)
    }
}
