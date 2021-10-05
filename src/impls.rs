use std::error;

use crate::{
    source,
    value::{self, Value},
};

impl Value for () {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.none()
    }
}

impl<'a> source::Source<'a> for () {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.none()
    }
}

impl<'a> source::TypedSource<'a, ()> for () {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&(), source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for bool {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.bool(*self)
    }
}

impl<'a> source::TypedSource<'a, bool> for bool {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&bool, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for u8 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(*self as u64)
    }
}

impl<'a> source::TypedSource<'a, u8> for u8 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&u8, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for i8 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(*self as i64)
    }
}

impl<'a> source::TypedSource<'a, i8> for i8 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&i8, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for u16 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(*self as u64)
    }
}

impl<'a> source::TypedSource<'a, u16> for u16 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&u16, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for i16 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(*self as i64)
    }
}

impl<'a> source::TypedSource<'a, i16> for i16 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&i16, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for u32 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(*self as u64)
    }
}

impl<'a> source::TypedSource<'a, u32> for u32 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&u32, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for i32 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(*self as i64)
    }
}

impl<'a> source::TypedSource<'a, i32> for i32 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&i32, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for u64 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(*self)
    }
}

impl<'a> source::TypedSource<'a, u64> for u64 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&u64, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i64 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i64(*self)
    }
}

impl<'a> source::Source<'a> for i64 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(*self)
    }
}

impl<'a> source::TypedSource<'a, i64> for i64 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&i64, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for u128 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u128(*self)
    }
}

impl<'a> source::TypedSource<'a, u128> for u128 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&u128, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for i128 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i128(*self)
    }
}

impl<'a> source::TypedSource<'a, i128> for i128 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&i128, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for f32 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.f64(*self as f64)
    }
}

impl<'a> source::TypedSource<'a, f32> for f32 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&f32, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for f64 {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.f64(*self)
    }
}

impl<'a> source::TypedSource<'a, f64> for f64 {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&f64, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for str {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.str(self)
    }
}

impl<'a> source::Source<'a> for str {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.str(self)
    }
}

impl<'a> source::TypedSource<'a, str> for str {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(self)
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

impl<'a> source::Source<'a> for String {
    fn stream<'b, S>(&mut self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.str(self.for_all())
    }
}

impl<'a> source::TypedSource<'a, str> for String {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(&**self)
    }
}

impl<'a> source::TypedSource<'a, str> for &'a String {
    type Error = source::Impossible;

    fn stream_to_value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(&**self)
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

impl<T, const N: usize> Value for [T; N]
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

impl Value for dyn error::Error + 'static {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.error(self)
    }
}
