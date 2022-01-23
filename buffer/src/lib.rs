use std::borrow::Cow;

use sval::{
    receiver::{Display, Receiver},
    source::{self, Source, Stream, ValueSource},
    value::Value,
    Result,
};

use sval_fmt as fmt;

pub fn buffer<'a>(receiver: impl BufferReceiver<'a>, mut source: impl Source<'a>) -> Result {
    struct Extract<'a, R> {
        buffer: Buffer<'a>,
        receiver: Option<R>,
    }

    impl<'a, R: BufferReceiver<'a>> Extract<'a, R> {
        #[inline]
        fn top_level_receiver(&mut self) -> Option<R> {
            if self.buffer.is_empty() {
                self.receiver.take()
            } else {
                None
            }
        }
    }

    impl<'a, R: BufferReceiver<'a>> Receiver<'a> for Extract<'a, R> {
        fn unstructured<D: Display>(&mut self, value: D) -> Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                struct Adapter<D>(fmt::Value<D>);

                impl<'a, D: Display> Source<'a> for Adapter<D> {
                    fn stream_resume<'b, R: Receiver<'b>>(
                        &mut self,
                        mut receiver: R,
                    ) -> Result<Stream>
                    where
                        'a: 'b,
                    {
                        receiver.unstructured(self.0.get())?;

                        Ok(Stream::Done)
                    }
                }

                impl<'a, D: Display> ValueSource<'a, fmt::Value<D>, source::Impossible> for Adapter<D> {
                    type Error = source::Impossible;

                    fn take(&mut self) -> Result<&fmt::Value<D>, source::TakeError<Self::Error>> {
                        Ok(&self.0)
                    }
                }

                receiver.value_source(Adapter(fmt::Value::new(value)))
            } else {
                self.buffer.push(Token::Display(value.to_string()));

                Ok(())
            }
        }

        fn none(&mut self) -> Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(&Option::None::<()>)
            } else {
                self.buffer.push(Token::None);

                Ok(())
            }
        }

        fn str<'v: 'a, V: ValueSource<'v, str>>(&mut self, mut value: V) -> Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                match value.try_take_ref() {
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

        fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                value.stream(self)
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

pub trait BufferReceiver<'a> {
    fn value_source<'v: 'a, T: Value + ?Sized, R: Value + ?Sized + 'v, S: ValueSource<'v, T, R>>(
        &mut self,
        value: S,
    ) -> Result;
}

impl<'a, 'b, R: BufferReceiver<'a> + ?Sized> BufferReceiver<'a> for &'b mut R {
    fn value_source<'v: 'a, T: Value + ?Sized, U: Value + ?Sized + 'v, S: ValueSource<'v, T, U>>(
        &mut self,
        value: S,
    ) -> Result {
        (**self).value_source(value)
    }
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

impl<'a> Value for Buffer<'a> {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        for mut token in &self.buf {
            token.stream_to_end(&mut receiver)?;
        }

        Ok(())
    }
}

impl<'a> Source<'a> for Buffer<'a> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result<Stream>
    where
        'a: 'b,
    {
        match self.buf.get(self.idx) {
            Some(mut token) => {
                self.idx += 1;

                token.stream_to_end(&mut receiver)?;

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

enum Token<'a> {
    Display(String),
    Str(Cow<'a, str>),
    None,
    MapBegin(Option<usize>),
    MapEnd,
    MapKeyBegin,
    MapKeyEnd,
    MapValueBegin,
    MapValueEnd,
    SeqBegin(Option<usize>),
    SeqEnd,
    SeqElemBegin,
    SeqElemEnd,
}

impl<'a, 'b> Source<'a> for &'b Token<'a> {
    fn stream_resume<'c, R: Receiver<'c>>(&mut self, mut receiver: R) -> Result<Stream>
    where
        'a: 'c,
    {
        match *self {
            Token::Str(Cow::Borrowed(value)) => receiver.str(*value)?,
            Token::Str(Cow::Owned(value)) => receiver.str(source::for_all(value))?,
            Token::Display(value) => receiver.unstructured(value)?,
            Token::None => receiver.none()?,
            Token::MapBegin(len) => receiver.map_begin(*len)?,
            Token::MapEnd => receiver.map_end()?,
            Token::MapKeyBegin => receiver.map_key_begin()?,
            Token::MapKeyEnd => receiver.map_key_end()?,
            Token::MapValueBegin => receiver.map_value_begin()?,
            Token::MapValueEnd => receiver.map_value_end()?,
            Token::SeqBegin(len) => receiver.seq_begin(*len)?,
            Token::SeqEnd => receiver.seq_end()?,
            Token::SeqElemBegin => receiver.seq_elem_begin()?,
            Token::SeqElemEnd => receiver.seq_elem_end()?,
        };

        Ok(Stream::Done)
    }
}
