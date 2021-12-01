use crate::{
    source::{self, ValueSource},
    std::fmt::{self, Write},
    Receiver, Result, Source, Value,
};

pub fn digits(digits: &impl fmt::Display) -> Result<&Digits> {
    Digits::new(digits)
}

pub fn digits_unchecked(digits: &impl fmt::Display) -> &Digits {
    Digits::new_unchecked(digits)
}

#[repr(transparent)]
pub struct Digits(dyn fmt::Display);

impl Digits {
    pub fn new(digits: &impl fmt::Display) -> Result<&Digits> {
        Inspect::read(&digits)
            .ok_or(crate::Error)
            .map(|_| Self::new_unchecked(digits))
    }

    pub fn new_unchecked(digits: &impl fmt::Display) -> &Digits {
        // SAFETY: `Digits` and `dyn fmt::Display` have the same ABI
        unsafe { &*(digits as *const dyn fmt::Display as *const Digits) }
    }

    pub fn is_sign_negative(&self) -> bool {
        Inspect::read(&self.0)
            .map(|i| i.is_sign_negative)
            .unwrap_or(false)
    }

    pub fn is_sign_positive(&self) -> bool {
        Inspect::read(&self.0)
            .map(|i| !i.is_sign_negative)
            .unwrap_or(false)
    }

    pub fn is_integer(&self) -> bool {
        Inspect::read(&self.0)
            .map(|i| !i.is_fractional)
            .unwrap_or(false)
    }

    pub fn formatter(&self) -> Formatter {
        Formatter::new(self)
    }
}

impl Value for Digits {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.digits(self)
    }
}

pub struct Formatter<'a> {
    digits: &'a Digits,
    omit_positive_sign: bool,
}

impl<'a> Formatter<'a> {
    #[inline]
    fn new(digits: &'a Digits) -> Self {
        Formatter {
            digits,
            omit_positive_sign: false,
        }
    }

    pub fn omit_positive_sign(&mut self, omit: bool) -> &mut Self {
        self.omit_positive_sign = omit;
        self
    }
}

impl<'a> fmt::Display for Formatter<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Checked<'a, 'b> {
            inspect: Inspect,
            dst: &'a mut fmt::Formatter<'b>,
        }

        impl<'a, 'b> Write for Checked<'a, 'b> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                if let Some(s) = self.inspect.next(s) {
                    self.dst.write_str(s)
                } else {
                    Ok(())
                }
            }
        }

        let mut checked = Checked {
            inspect: Inspect {
                omit_positive_sign: self.omit_positive_sign,
                ..Default::default()
            },
            dst: f,
        };

        write!(&mut checked, "{}", &self.digits.0)?;

        if checked.inspect.finish() {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

impl<'a> fmt::Debug for Formatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.formatter(), f)
    }
}

impl fmt::Debug for Digits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

struct Inspect {
    state: State,
    is_sign_negative: bool,
    is_fractional: bool,
    omit_positive_sign: bool,
}

enum State {
    Sign,
    Digits(usize),
    Fractional(usize),
    Valid,
    Invalid,
}

impl Default for Inspect {
    #[inline]
    fn default() -> Self {
        Inspect {
            state: State::Sign,
            is_sign_negative: false,
            is_fractional: false,
            omit_positive_sign: false,
        }
    }
}

impl Inspect {
    #[inline]
    fn read(v: impl fmt::Display) -> Option<Self> {
        let mut inspect = Inspect::default();
        let _ = write!(&mut inspect, "{}", &v);

        if inspect.finish() {
            Some(inspect)
        } else {
            None
        }
    }

    #[inline]
    fn next<'a>(&mut self, s: &'a str) -> Option<&'a str> {
        let mut slice_from = 0..s.len();

        for b in s.as_bytes() {
            match (&mut self.state, b) {
                (State::Sign, b'-') => {
                    self.is_sign_negative = true;
                    self.state = State::Digits(0);
                }
                (State::Sign, b'+') => {
                    self.state = State::Digits(0);

                    if self.omit_positive_sign {
                        slice_from.start = 1;
                    }
                }
                (State::Sign, b'0'..=b'9') => {
                    self.state = State::Digits(1);
                }
                (State::Digits(ref mut c), b'0'..=b'9') => {
                    *c += 1;
                }
                (State::Digits(c), b'.') if *c > 0 => {
                    self.state = State::Fractional(0);
                }
                (State::Fractional(ref mut c), b'0') => {
                    *c += 1;
                }
                (State::Fractional(ref mut c), b'1'..=b'9') => {
                    self.is_fractional = true;
                    *c += 1;
                }
                _ => {
                    self.state = State::Invalid;
                }
            }
        }

        s.get(slice_from)
    }

    #[inline]
    fn finish(&mut self) -> bool {
        match self.state {
            State::Digits(c) | State::Fractional(c) if c > 0 => {
                self.state = State::Valid;
                true
            }
            _ => {
                self.state = State::Invalid;
                false
            }
        }
    }
}

impl Write for Inspect {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = self.next(s);
        Ok(())
    }
}

impl Value for u8 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u8(*self)
    }
}

impl<'a> Source<'a> for u8 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u8(*self)
    }
}

impl<'a> ValueSource<'a, u8> for u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u8, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for i8 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i8(*self)
    }
}

impl<'a> Source<'a> for i8 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i8(*self)
    }
}

impl<'a> ValueSource<'a, i8> for i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i8, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for u16 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u16(*self)
    }
}

impl<'a> Source<'a> for u16 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u16(*self)
    }
}

impl<'a> ValueSource<'a, u16> for u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u16, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for i16 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i16(*self)
    }
}

impl<'a> Source<'a> for i16 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i16(*self)
    }
}

impl<'a> ValueSource<'a, i16> for i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i16, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for u32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u32(*self)
    }
}

impl<'a> Source<'a> for u32 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u32(*self)
    }
}

impl<'a> ValueSource<'a, u32> for u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for i32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i32(*self)
    }
}

impl<'a> Source<'a> for i32 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i32(*self)
    }
}

impl<'a> ValueSource<'a, i32> for i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for u64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u64(*self)
    }
}

impl<'a> Source<'a> for u64 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u64(*self)
    }
}

impl<'a> ValueSource<'a, u64> for u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for i64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i64(*self)
    }
}

impl<'a> Source<'a> for i64 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i64(*self)
    }
}

impl<'a> ValueSource<'a, i64> for i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for u128 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u128(*self)
    }
}

impl<'a> Source<'a> for u128 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.u128(*self)
    }
}

impl<'a> ValueSource<'a, u128> for u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u128, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for i128 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i128(*self)
    }
}

impl<'a> Source<'a> for i128 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.i128(*self)
    }
}

impl<'a> ValueSource<'a, i128> for i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i128, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for f32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.f32(*self)
    }
}

impl<'a> Source<'a> for f32 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.f32(*self)
    }
}

impl<'a> ValueSource<'a, f32> for f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&f32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl Value for f64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.f64(*self)
    }
}

impl<'a> Source<'a> for f64 {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.f64(*self)
    }
}

impl<'a> ValueSource<'a, f64> for f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&f64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, Digits> for &'a f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a Digits, source::TakeRefError<&Digits, Self::Error>> {
        Ok(digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, Digits> for f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        Ok(digits_unchecked(self))
    }
}

impl<'a> ValueSource<'a, Digits> for &'a str {
    type Error = crate::Error;

    #[inline]
    fn take(&mut self) -> Result<&Digits, source::TakeError<Self::Error>> {
        digits(self).map_err(source::TakeError::from_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        for s in &[
            "0",
            "0.0",
            "+0",
            "+0.0",
            "-0",
            "-0.0",
            "1",
            "1.00000000000000000000",
            "0000000000000000000001",
            "17658364437897823498497675849928784764328764578395677652",
            "213465478765433985883447878.493754376829326274588576377682755109",
        ] {
            assert!(Digits::new(s).is_ok());
            assert_eq!(*s, Digits::new(s).unwrap().to_string());
        }
    }

    #[test]
    fn invalid() {
        for s in &[
            "",
            "+",
            "-",
            ".",
            "+.0",
            "-.0",
            ".1",
            "a",
            "0xAF",
            "1.83475e9",
        ] {
            assert!(Digits::new(s).is_err());
        }
    }

    #[test]
    fn is_sign_positive() {
        for s in &[
            "0",
            "1",
            "+0",
            "+1",
            "768392057567898765434567898765456787654.4854637829837465456781234",
        ] {
            assert!(Digits::new(s).unwrap().is_sign_positive());
            assert!(!Digits::new(s).unwrap().is_sign_negative());
        }
    }

    #[test]
    fn is_sign_negative() {
        for s in &[
            "-0",
            "-1",
            "-233455674367678867956989864871260.549475768256875605679857",
        ] {
            assert!(Digits::new(s).unwrap().is_sign_negative());
            assert!(!Digits::new(s).unwrap().is_sign_positive());
        }
    }

    #[test]
    fn integers() {
        for s in &[
            "0",
            "1",
            "4375786847654973496709849345875936753407",
            "-0",
            "+57348",
            "-3256453874327",
            "0.0",
            "1.000000",
            "8745.0000000000000000000000",
            "000001.0000",
        ] {
            assert!(Digits::new(s).unwrap().is_integer());
        }
    }

    #[test]
    fn fractionals() {
        for s in &[
            "0.1",
            "-1.45873",
            "48753.00000000000000000000000000000000000000000000001",
        ] {
            assert!(!Digits::new(s).unwrap().is_integer());
        }
    }

    #[test]
    fn format_omit_sign_positive() {
        for (s, expected) in &[
            ("+0", "0"),
            ("+0.0", "0.0"),
            ("+328573465437", "328573465437"),
            ("348795237896387", "348795237896387"),
        ] {
            assert_eq!(
                *expected,
                Digits::new(s)
                    .unwrap()
                    .formatter()
                    .omit_positive_sign(true)
                    .to_string()
            );
        }
    }
}
