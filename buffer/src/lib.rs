use std::borrow::Cow;

use sval_generic_api::{
    for_all,
    receiver::{self, Display, Receiver},
    source::{self, Source, Stream, ValueSource},
    value::Value,
    Result,
};

use sval_generic_api_fmt as fmt;

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

    impl<'a, R: BufferReceiver<'a>> Extract<'a, R> {
        fn top_level_receiver(&mut self) -> Option<R> {
            if self.buffer.is_empty() {
                self.receiver.take()
            } else {
                None
            }
        }
    }

    impl<'a, R: BufferReceiver<'a>> Receiver<'a> for Extract<'a, R> {
        fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
            if let Some(receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                value.stream(self)
            }
        }

        fn display<D: Display>(&mut self, value: D) -> Result {
            if let Some(receiver) = self.top_level_receiver() {
                // TODO: Can't actually use `ValueSource` for this...
                // The lifetime is too short, we typically need `'static`...
                receiver.value_source(source::for_all(&fmt::display(value)))
            } else {
                self.buffer.push(Token::Display(value.to_string()));

                Ok(())
            }
        }

        fn none(&mut self) -> Result {
            if let Some(receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                self.buffer.push(Token::None);

                Ok(())
            }
        }

        fn bool(&mut self, value: bool) -> Result {
            if let Some(receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                self.buffer.push(Token::Bool(value));

                Ok(())
            }
        }

        fn str<'v: 'a, V: ValueSource<'v, str>>(&mut self, mut value: V) -> Result {
            if let Some(receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                match value.take_ref() {
                    Ok(v) => {
                        self.buffer.push(Token::Str(Cow::Borrowed(v)));
    
                        Ok(())
                    }
                    Err(v) => {
                        self.buffer
                            .push(Token::Str(Cow::Owned(v.into_result()?.to_owned())));
    
                        Ok(())
                    }
                }
            }
        }

        fn map_begin(&mut self, len: Option<usize>) -> Result {
            self.buffer.push(Token::MapBegin(len));

            Ok(())
        }

        fn map_end(&mut self) -> Result {
            self.buffer.push(Token::MapEnd);

            Ok(())
        }

        fn map_key_begin(&mut self) -> Result {
            self.buffer.push(Token::MapKeyBegin);

            Ok(())
        }

        fn map_key_end(&mut self) -> Result {
            self.buffer.push(Token::MapKeyEnd);

            Ok(())
        }

        fn map_value_begin(&mut self) -> Result {
            self.buffer.push(Token::MapValueBegin);

            Ok(())
        }

        fn map_value_end(&mut self) -> Result {
            self.buffer.push(Token::MapValueEnd);

            Ok(())
        }

        fn seq_begin(&mut self, len: Option<usize>) -> Result {
            self.buffer.push(Token::SeqBegin(len));

            Ok(())
        }

        fn seq_end(&mut self) -> Result {
            self.buffer.push(Token::SeqEnd);

            Ok(())
        }

        fn seq_elem_begin(&mut self) -> Result {
            self.buffer.push(Token::SeqElemBegin);

            Ok(())
        }

        fn seq_elem_end(&mut self) -> Result {
            self.buffer.push(Token::SeqElemEnd);

            Ok(())
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
    Display(String),
    Str(Cow<'a, str>),
    Bool(bool),
    None,
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
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result<Stream>
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

                Ok(Stream::Yield)
            }
            None => Ok(Stream::Done),
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
