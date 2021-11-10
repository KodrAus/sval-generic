use std::borrow::Cow;

use sval_generic_api::{
    for_all,
    receiver::{self, Display, Receiver},
    source::{self, Source, StreamState, ValueSource},
    value::Value,
    Result,
};

pub trait BufferReceiver<'a> {
    fn value_source<'v: 'a, T: Value + ?Sized + 'v, S: ValueSource<'v, T>>(
        &mut self,
        value: S,
    ) -> Result;
}

impl<'a, 'b, R: BufferReceiver<'a> + ?Sized> BufferReceiver<'a> for &'b mut R {
    fn value_source<'v: 'a, T: Value + ?Sized + 'v, S: ValueSource<'v, T>>(
        &mut self,
        value: S,
    ) -> Result {
        (**self).value_source(value)
    }
}

pub fn buffer<'a>(receiver: impl BufferReceiver<'a>, mut source: impl Source<'a>) -> Result {
    struct Extract<'a, R> {
        buffer: Buffer<'a>,
        receiver: Option<R>,
    }

    impl<'a, R: BufferReceiver<'a>> Receiver<'a> for Extract<'a, R> {
        fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
            // If we receive a top-level value then we don't need to do any buffering
            // We can just surface it to the receiver as-is
            if self.buffer.is_empty() {
                if let Some(mut receiver) = self.receiver.take() {
                    return receiver.value_source(value);
                }
            }

            //value.stream(self)
            receiver::unsupported()
        }

        fn display<D: Display>(&mut self, _: D) -> Result {
            receiver::unsupported()
        }

        fn none(&mut self) -> Result {
            receiver::unsupported()
        }

        fn bool(&mut self, value: bool) -> Result {
            /*self.buffer.push(Token::Bool(value));

            Ok(())*/
            receiver::unsupported()
        }

        fn str<'v: 'a, V: ValueSource<'v, str>>(&mut self, mut value: V) -> Result {
            /*match value.take_ref() {
                Ok(v) => {
                    self.buffer.push(Token::Str(Cow::Borrowed(v)));
                    Ok(())
                }
                Err(v) => {
                    self.buffer
                        .push(Token::Str(Cow::Owned(v.into_result()?.to_owned())));
                    Ok(())
                }
            }*/
            receiver::unsupported()
        }

        fn map_begin(&mut self, _: Option<usize>) -> Result {
            receiver::unsupported()
        }

        fn map_end(&mut self) -> Result {
            receiver::unsupported()
        }

        fn map_key_begin(&mut self) -> Result {
            receiver::unsupported()
        }

        fn map_key_end(&mut self) -> Result {
            receiver::unsupported()
        }

        fn map_value_begin(&mut self) -> Result {
            receiver::unsupported()
        }

        fn map_value_end(&mut self) -> Result {
            receiver::unsupported()
        }

        fn seq_begin(&mut self, _: Option<usize>) -> Result {
            receiver::unsupported()
        }

        fn seq_end(&mut self) -> Result {
            receiver::unsupported()
        }

        fn seq_elem_begin(&mut self) -> Result {
            receiver::unsupported()
        }

        fn seq_elem_end(&mut self) -> Result {
            receiver::unsupported()
        }
    }

    let mut extract = Extract {
        buffer: Buffer::new(),
        receiver: Some(receiver),
    };

    source.stream_to_end(&mut extract)?;

    if let Some(mut receiver) = extract.receiver.take() {
        receiver.value_source(extract.buffer)?;
    }

    Ok(())
}

struct Buffer<'a> {
    buf: Vec<Token<'a>>,
    idx: usize,
}

impl<'a> Buffer<'a> {
    fn new() -> Self {
        Buffer {
            buf: Vec::new(),
            idx: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    fn push(&mut self, token: Token<'a>) {
        self.buf.push(token);
    }
}

enum Token<'a> {
    Str(Cow<'a, str>),
    Bool(bool),
}

impl<'a> Value for Buffer<'a> {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        for token in &self.buf {
            match token {
                Token::Str(Cow::Borrowed(value)) => receiver.str(*value)?,
                Token::Str(Cow::Owned(value)) => receiver.str(for_all(value))?,
                Token::Bool(value) => receiver.bool(*value)?,
            }
        }

        Ok(())
    }
}

impl<'a> Source<'a> for Buffer<'a> {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result<StreamState>
    where
        'a: 'b,
    {
        match self.buf.get(self.idx) {
            Some(token) => {
                self.idx += 1;

                match token {
                    Token::Str(Cow::Borrowed(value)) => receiver.str(*value)?,
                    Token::Str(Cow::Owned(value)) => receiver.str(for_all(value))?,
                    Token::Bool(value) => receiver.bool(*value)?,
                }

                Ok(StreamState::Yield)
            }
            None => Ok(StreamState::Done),
        }
    }
}

impl<'a> ValueSource<'a, Buffer<'a>> for Buffer<'a> {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Buffer<'a>, source::TakeError<Self::Error>> {
        Ok(self)
    }
}
