use crate::{
    source,
    std::fmt::{self, Write},
    Receiver, Result, Source, Value,
};

#[repr(transparent)]
pub struct Number(dyn fmt::Display);

impl Number {
    pub fn new<'a>(num: &'a impl fmt::Display) -> &'a Number {
        let num: &'a dyn fmt::Display = num;

        // SAFETY: `Number` and `dyn fmt::Display` have the same ABI
        unsafe { &*(num as *const dyn fmt::Display as *const Number) }
    }
}

impl Value for Number {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.num(self)
    }
}

impl<'a> Source<'a> for Number {
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
        receiver.num(self)
    }
}

impl<'a> source::ValueSource<'a, Number> for Number {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Number, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Check<'a, 'b> {
            state: State,
            output: &'a mut fmt::Formatter<'b>,
        }

        enum State {
            Sign,
            Digits(usize),
            Fractional(usize),
        }

        impl<'a, 'b> Write for Check<'a, 'b> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                for b in s.as_bytes() {
                    match (&mut self.state, b) {
                        (State::Sign, b'-') => {
                            self.state = State::Digits(0);
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
                        (State::Fractional(ref mut c), b'0'..=b'9') => {
                            *c += 1;
                        }
                        _ => return Err(fmt::Error),
                    }
                }

                self.output.write_str(s)
            }
        }

        let mut check = Check {
            state: State::Sign,
            output: f,
        };

        write!(&mut check, "{}", &self.0)?;

        match check.state {
            State::Digits(c) | State::Fractional(c) if c > 0 => Ok(()),
            _ => Err(fmt::Error),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
