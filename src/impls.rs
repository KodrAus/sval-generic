use std::error;

use crate::{
    reference,
    source::{self, Source},
};

impl Source for () {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.none()
    }
}

impl<'a> reference::SourceRef<'a> for () {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.none()
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, ()> for () {
    fn get(&self) -> &() {
        self
    }

    fn try_unwrap(self) -> Option<&'a ()> {
        None
    }
}

impl Source for bool {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.bool(*self)
    }
}

impl<'a> reference::SourceRef<'a> for bool {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.bool(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, bool> for bool {
    fn get(&self) -> &bool {
        self
    }

    fn try_unwrap(self) -> Option<&'a bool> {
        None
    }
}

impl Source for u8 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.u64(*self as u64)
    }
}

impl<'a> reference::SourceRef<'a> for u8 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.u64(self as u64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, u8> for u8 {
    fn get(&self) -> &u8 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u8> {
        None
    }
}

impl Source for i8 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.i64(*self as i64)
    }
}

impl<'a> reference::SourceRef<'a> for i8 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.i64(self as i64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, i8> for i8 {
    fn get(&self) -> &i8 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i8> {
        None
    }
}

impl Source for u16 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.u64(*self as u64)
    }
}

impl<'a> reference::SourceRef<'a> for u16 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.u64(self as u64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, u16> for u16 {
    fn get(&self) -> &u16 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u16> {
        None
    }
}

impl Source for i16 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.i64(*self as i64)
    }
}

impl<'a> reference::SourceRef<'a> for i16 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.i64(self as i64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, i16> for i16 {
    fn get(&self) -> &i16 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i16> {
        None
    }
}

impl Source for u32 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.u64(*self as u64)
    }
}

impl<'a> reference::SourceRef<'a> for u32 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.u64(self as u64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, u32> for u32 {
    fn get(&self) -> &u32 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u32> {
        None
    }
}

impl Source for i32 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.i64(*self as i64)
    }
}

impl<'a> reference::SourceRef<'a> for i32 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.i64(self as i64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, i32> for i32 {
    fn get(&self) -> &i32 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i32> {
        None
    }
}

impl Source for u64 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.u64(*self)
    }
}

impl<'a> reference::SourceRef<'a> for u64 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.u64(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, u64> for u64 {
    fn get(&self) -> &u64 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u64> {
        None
    }
}

impl Source for i64 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.i64(*self)
    }
}

impl<'a> reference::SourceRef<'a> for i64 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.i64(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, i64> for i64 {
    fn get(&self) -> &i64 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i64> {
        None
    }
}

impl Source for u128 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.u128(*self)
    }
}

impl<'a> reference::SourceRef<'a> for u128 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.u128(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, u128> for u128 {
    fn get(&self) -> &u128 {
        self
    }

    fn try_unwrap(self) -> Option<&'a u128> {
        None
    }
}

impl Source for i128 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.i128(*self)
    }
}

impl<'a> reference::SourceRef<'a> for i128 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.i128(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, i128> for i128 {
    fn get(&self) -> &i128 {
        self
    }

    fn try_unwrap(self) -> Option<&'a i128> {
        None
    }
}

impl Source for f32 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.f64(*self as f64)
    }
}

impl<'a> reference::SourceRef<'a> for f32 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.f64(self as f64)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, f32> for f32 {
    fn get(&self) -> &f32 {
        self
    }

    fn try_unwrap(self) -> Option<&'a f32> {
        None
    }
}

impl Source for f64 {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.f64(*self)
    }
}

impl<'a> reference::SourceRef<'a> for f64 {
    fn stream<'b, S>(self, mut stream: S) -> source::Result
    where
        'a: 'b,
        S: source::Stream<'b>,
    {
        stream.f64(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

impl<'a> reference::TypedRef<'a, f64> for f64 {
    fn get(&self) -> &f64 {
        self
    }

    fn try_unwrap(self) -> Option<&'a f64> {
        None
    }
}

impl Source for str {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.str(self)
    }

    fn to_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl Source for String {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.str(&**self)
    }

    fn to_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> reference::TypedRef<'a, str> for &'a String {
    fn get(&self) -> &str {
        &**self
    }

    fn try_unwrap(self) -> Option<&'a str> {
        Some(&**self)
    }
}

impl<T> Source for Option<T>
where
    T: Source,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        match self {
            Some(v) => v.stream(stream),
            None => stream.none(),
        }
    }
}

impl<T> Source for [T]
where
    T: Source,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_elem(elem)?;
        }

        stream.seq_end()
    }
}

impl<T, const N: usize> Source for [T; N]
where
    T: Source,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_elem(elem)?;
        }

        stream.seq_end()
    }
}

impl<T> Source for Vec<T>
where
    T: Source,
{
    fn stream<'a, S>(&'a self, stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        (&**self).stream(stream)
    }
}

impl<T, U> Source for (T, U)
where
    T: Source,
    U: Source,
{
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.seq_begin(Some(2))?;
        stream.seq_elem(&self.0)?;
        stream.seq_elem(&self.1)?;
        stream.seq_end()
    }
}

impl<T: ?Sized> Source for Box<T>
where
    T: Source,
{
    fn stream<'a, S>(&'a self, stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        (**self).stream(stream)
    }
}

impl Source for dyn error::Error + 'static {
    fn stream<'a, S>(&'a self, mut stream: S) -> source::Result
    where
        S: source::Stream<'a>,
    {
        stream.error(self)
    }
}
