use std::error;

use crate::{
    stream,
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

impl<'a> stream::ValueRef<'a> for () {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.none()
    }
}

impl<'a> stream::Ref<'a, ()> for () {
    fn get(&self) -> &() {
        self
    }

    fn try_unwrap(self) -> Option<&'a ()> {
        None
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

impl<'a> stream::ValueRef<'a> for bool {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.bool(self)
    }
}

impl<'a> stream::Ref<'a, bool> for bool {
    fn get(&self) -> &bool {
        self
    }

    fn try_unwrap(self) -> Option<&'a bool> {
        None
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

impl<'a> stream::ValueRef<'a> for u8 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(self as u64)
    }
}

impl<'a> stream::Ref<'a, u8> for u8 {
    fn get(&self) -> &u8 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u8> {
        None
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

impl<'a> stream::ValueRef<'a> for i8 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(self as i64)
    }
}

impl<'a> stream::Ref<'a, i8> for i8 {
    fn get(&self) -> &i8 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i8> {
        None
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

impl<'a> stream::ValueRef<'a> for u16 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(self as u64)
    }
}

impl<'a> stream::Ref<'a, u16> for u16 {
    fn get(&self) -> &u16 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u16> {
        None
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

impl<'a> stream::ValueRef<'a> for i16 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(self as i64)
    }
}

impl<'a> stream::Ref<'a, i16> for i16 {
    fn get(&self) -> &i16 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i16> {
        None
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

impl<'a> stream::ValueRef<'a> for u32 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(self as u64)
    }
}

impl<'a> stream::Ref<'a, u32> for u32 {
    fn get(&self) -> &u32 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u32> {
        None
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

impl<'a> stream::ValueRef<'a> for i32 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(self as i64)
    }
}

impl<'a> stream::Ref<'a, i32> for i32 {
    fn get(&self) -> &i32 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i32> {
        None
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

impl<'a> stream::ValueRef<'a> for u64 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u64(self)
    }
}

impl<'a> stream::Ref<'a, u64> for u64 {
    fn get(&self) -> &u64 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u64> {
        None
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

impl<'a> stream::ValueRef<'a> for i64 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i64(self)
    }
}

impl<'a> stream::Ref<'a, i64> for i64 {
    fn get(&self) -> &i64 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i64> {
        None
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

impl<'a> stream::ValueRef<'a> for u128 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.u128(self)
    }
}

impl<'a> stream::Ref<'a, u128> for u128 {
    fn get(&self) -> &u128 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u128> {
        None
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

impl<'a> stream::ValueRef<'a> for i128 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.i128(self)
    }
}

impl<'a> stream::Ref<'a, i128> for i128 {
    fn get(&self) -> &i128 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i128> {
        None
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

impl<'a> stream::ValueRef<'a> for f32 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.f64(self as f64)
    }
}

impl<'a> stream::Ref<'a, f32> for f32 {
    fn get(&self) -> &f32 {
        self
    }

    fn try_unwrap(self) -> Option<&'a f32> {
        None
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

impl<'a> stream::ValueRef<'a> for f64 {
    fn stream<'b, S>(self, mut stream: S) -> value::Result
    where
        'a: 'b,
        S: value::Stream<'b>,
    {
        stream.f64(self)
    }
}

impl<'a> stream::Ref<'a, f64> for f64 {
    fn get(&self) -> &f64 {
        self
    }

    fn try_unwrap(self) -> Option<&'a f64> {
        None
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

impl<'a> stream::Ref<'a, str> for &'a String {
    fn get(&self) -> &str {
        &**self
    }

    fn try_unwrap(self) -> Option<&'a str> {
        Some(&**self)
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
