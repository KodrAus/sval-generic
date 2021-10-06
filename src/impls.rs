use std::{borrow::Cow, error};

use crate::{source, Receiver, Source, Value};

impl Value for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.none()
    }
}

impl<'a> Source<'a> for () {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.none()
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, ()> for () {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&(), source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }
}

impl<'a> Source<'a> for bool {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.bool(*self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, bool> for bool {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&bool, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u8 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u64(*self as u64)
    }
}

impl<'a> Source<'a> for u8 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u64(*self as u64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, u8> for u8 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&u8, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i8 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i64(*self as i64)
    }
}

impl<'a> Source<'a> for i8 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i64(*self as i64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, i8> for i8 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&i8, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u16 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u64(*self as u64)
    }
}

impl<'a> Source<'a> for u16 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u64(*self as u64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, u16> for u16 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&u16, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i16 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i64(*self as i64)
    }
}

impl<'a> Source<'a> for i16 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i64(*self as i64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, i16> for i16 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&i16, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u64(*self as u64)
    }
}

impl<'a> Source<'a> for u32 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u64(*self as u64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, u32> for u32 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&u32, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i64(*self as i64)
    }
}

impl<'a> Source<'a> for i32 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i64(*self as i64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, i32> for i32 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&i32, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u64(*self)
    }
}

impl<'a> Source<'a> for u64 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u64(*self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, u64> for u64 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&u64, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i64(*self)
    }
}

impl<'a> Source<'a> for i64 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i64(*self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, i64> for i64 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&i64, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u128 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u128(*self)
    }
}

impl<'a> Source<'a> for u128 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u128(*self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, u128> for u128 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&u128, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i128 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i128(*self)
    }
}

impl<'a> Source<'a> for i128 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i128(*self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, i128> for i128 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&i128, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for f32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.f64(*self as f64)
    }
}

impl<'a> Source<'a> for f32 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.f64(*self as f64)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, f32> for f32 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&f32, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for f64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.f64(*self)
    }
}

impl<'a> Source<'a> for f64 {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.f64(*self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, f64> for f64 {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&f64, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for str {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.str(self)
    }
}

impl<'a> Source<'a> for str {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.str(self)
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, str> for str {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(self)
    }
}

impl Value for String {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.str(&**self)
    }

    fn to_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Source<'a> for String {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.str(crate::for_all(self))
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, str> for String {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(&**self)
    }

    fn value_owned(&mut self) -> Result<String, source::ToValueError<Self::Error>> {
        Ok(std::mem::take(self))
    }
}

impl<'a> source::ValueSource<'a, str> for &'a String {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(&**self)
    }
}

impl<'a> Value for Cow<'a, str> {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> crate::Result {
        receiver.str(&**self)
    }

    fn to_str(&self) -> Option<&str> {
        if let Cow::Borrowed(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<'a> Source<'a> for Cow<'a, str> {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        match self {
            Cow::Borrowed(v) => receiver.str(v),
            Cow::Owned(v) => receiver.str(crate::for_all(v)),
        }
    }

    fn is_map_hint(&self) -> Option<bool> {
        Some(false)
    }

    fn is_seq_hint(&self) -> Option<bool> {
        Some(false)
    }
}

impl<'a> source::ValueSource<'a, str> for Cow<'a, str> {
    type Error = source::Impossible;

    fn value(&mut self) -> Result<&str, source::ToValueError<Self::Error>> {
        Ok(&**self)
    }

    fn value_ref(&mut self) -> Result<&'a str, source::ToRefError<&str, Self::Error>> {
        match self {
            Cow::Borrowed(v) => Ok(v),
            Cow::Owned(v) => Err(source::ToRefError::from_value(v)),
        }
    }

    fn value_owned(&mut self) -> Result<String, source::ToValueError<Self::Error>> {
        Ok(std::mem::take(self).into_owned())
    }
}

impl<T> Value for Option<T>
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        match self {
            Some(v) => v.stream(receiver),
            None => receiver.none(),
        }
    }
}

impl<T> Value for [T]
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T, const N: usize> Value for [T; N]
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T> Value for Vec<T>
where
    T: Value,
{
    fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
        (&**self).stream(stream)
    }
}

impl<T, U> Value for (T, U)
where
    T: Value,
    U: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(2))?;
        receiver.seq_elem(&self.0)?;
        receiver.seq_elem(&self.1)?;
        receiver.seq_end()
    }
}

impl<T: ?Sized> Value for Box<T>
where
    T: Value,
{
    fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
        (**self).stream(stream)
    }
}

impl Value for dyn error::Error + 'static {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.error(self)
    }
}
