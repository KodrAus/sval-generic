#![no_std]

extern crate alloc;

use alloc::{
    borrow::{Cow, ToOwned},
    string::{String, ToString},
    vec::Vec,
};

use sval::data::{Bytes, Tag, Text};
use sval::{Result, Source, ValueSource};

use sval_fmt as fmt;

pub fn buffer<'a>(
    receiver: impl BufferReceiver<'a>,
    mut source: impl sval::Source<'a>,
) -> sval::Result {
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

    impl<'a, R: BufferReceiver<'a>> sval::Receiver<'a> for Extract<'a, R> {
        fn value<'v: 'a, V: sval::Value + ?Sized + 'v>(&mut self, value: &'v V) -> sval::Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                value.stream(self)
            }
        }

        fn unstructured<D: sval::data::Display>(&mut self, value: D) -> sval::Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                struct Adapter<D>(fmt::Value<D>);

                impl<'a, D: sval::data::Display> sval::Source<'a> for Adapter<D> {
                    fn stream_resume<'b, R: sval::Receiver<'b>>(
                        &mut self,
                        mut receiver: R,
                    ) -> sval::Result<sval::source::Stream>
                    where
                        'a: 'b,
                    {
                        receiver.unstructured(self.0.get())?;

                        Ok(sval::source::Stream::Done)
                    }
                }

                impl<'a, D: sval::data::Display>
                    sval::ValueSource<'a, fmt::Value<D>, sval::source::Impossible> for Adapter<D>
                {
                    type Error = sval::source::Impossible;

                    fn take(
                        &mut self,
                    ) -> sval::Result<&fmt::Value<D>, sval::source::TakeError<Self::Error>>
                    {
                        Ok(&self.0)
                    }
                }

                receiver.value_source(Adapter(fmt::Value::new(value)))
            } else {
                self.buffer.push(Token::Unstructured(value.to_string()));

                Ok(())
            }
        }

        fn null(&mut self) -> sval::Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(&Option::None::<()>)
            } else {
                self.buffer.push(Token::Null);

                Ok(())
            }
        }

        fn str<'v: 'a, V: sval::ValueSource<'v, str>>(&mut self, mut value: V) -> sval::Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(value)
            } else {
                match value.try_take_ref() {
                    Ok(v) => {
                        self.buffer.push(Token::Str(Cow::Borrowed(v)));

                        Ok(())
                    }
                    Err(sval::source::TryTakeError::Fallback(v)) => {
                        self.buffer.push(Token::Str(Cow::Owned(v.to_owned())));

                        Ok(())
                    }
                    Err(sval::source::TryTakeError::Err(e)) => Err(e.into()),
                }
            }
        }

        fn text<'s: 'a, S: ValueSource<'s, Text>>(&mut self, mut text: S) -> sval::Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(text)
            } else {
                match text.try_take_ref() {
                    Ok(v) => {
                        self.buffer.push(Token::Text(Cow::Borrowed(v)));

                        Ok(())
                    }
                    Err(sval::source::TryTakeError::Fallback(v)) => {
                        self.buffer.push(Token::Text(Cow::Owned(v.to_owned())));

                        Ok(())
                    }
                    Err(sval::source::TryTakeError::Err(e)) => Err(e.into()),
                }
            }
        }

        fn bytes<'s: 'a, S: ValueSource<'s, Bytes>>(&mut self, mut bytes: S) -> sval::Result {
            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(bytes)
            } else {
                match bytes.try_take_ref() {
                    Ok(v) => {
                        self.buffer.push(Token::Bytes(Cow::Borrowed(v)));

                        Ok(())
                    }
                    Err(sval::source::TryTakeError::Fallback(v)) => {
                        self.buffer.push(Token::Bytes(Cow::Owned(v.to_owned())));

                        Ok(())
                    }
                    Err(sval::source::TryTakeError::Err(e)) => Err(e.into()),
                }
            }
        }

        fn tag<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
            let tag = tag.try_map_label(|mut label| match label.try_take_ref() {
                Ok(v) => Ok(Cow::Borrowed(v)),
                Err(sval::source::TryTakeError::Fallback(v)) => Ok(Cow::Owned(v.to_owned())),
                Err(sval::source::TryTakeError::Err(e)) => Err(e),
            })?;

            if let Some(mut receiver) = self.top_level_receiver() {
                receiver.value_source(sval::for_all(&tag))
            } else {
                self.buffer.push(Token::Tag(tag));

                Ok(())
            }
        }

        fn map_begin(&mut self, len: Option<u64>) -> sval::Result {
            self.buffer.push(Token::MapBegin(len));

            Ok(())
        }

        fn map_end(&mut self) -> sval::Result {
            self.buffer.push(Token::MapEnd);

            Ok(())
        }

        fn map_key_begin(&mut self) -> sval::Result {
            self.buffer.push(Token::MapKeyBegin);

            Ok(())
        }

        fn map_key_end(&mut self) -> sval::Result {
            self.buffer.push(Token::MapKeyEnd);

            Ok(())
        }

        fn map_value_begin(&mut self) -> sval::Result {
            self.buffer.push(Token::MapValueBegin);

            Ok(())
        }

        fn map_value_end(&mut self) -> sval::Result {
            self.buffer.push(Token::MapValueEnd);

            Ok(())
        }

        fn seq_begin(&mut self, len: Option<u64>) -> sval::Result {
            self.buffer.push(Token::SeqBegin(len));

            Ok(())
        }

        fn seq_end(&mut self) -> sval::Result {
            self.buffer.push(Token::SeqEnd);

            Ok(())
        }

        fn seq_elem_begin(&mut self) -> sval::Result {
            self.buffer.push(Token::SeqElemBegin);

            Ok(())
        }

        fn seq_elem_end(&mut self) -> sval::Result {
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
    fn value_source<
        'v: 'a,
        T: sval::Value + ?Sized,
        R: sval::Value + ?Sized + 'v,
        S: sval::ValueSource<'v, T, R>,
    >(
        &mut self,
        value: S,
    ) -> sval::Result;
}

impl<'a, 'b, R: BufferReceiver<'a> + ?Sized> BufferReceiver<'a> for &'b mut R {
    fn value_source<
        'v: 'a,
        T: sval::Value + ?Sized,
        U: sval::Value + ?Sized + 'v,
        S: sval::ValueSource<'v, T, U>,
    >(
        &mut self,
        value: S,
    ) -> sval::Result {
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

impl<'a> sval::Value for Buffer<'a> {
    fn stream<'b, R: sval::Receiver<'b>>(&'b self, mut receiver: R) -> sval::Result {
        for mut token in &self.buf {
            token.stream_to_end(&mut receiver)?;
        }

        Ok(())
    }
}

impl<'a> sval::Source<'a> for Buffer<'a> {
    fn stream_resume<'b, R: sval::Receiver<'b>>(
        &mut self,
        mut receiver: R,
    ) -> sval::Result<sval::source::Stream>
    where
        'a: 'b,
    {
        match self.buf.get(self.idx) {
            Some(mut token) => {
                self.idx += 1;

                token.stream_to_end(&mut receiver)?;

                Ok(sval::source::Stream::Yield)
            }
            None => Ok(sval::source::Stream::Done),
        }
    }
}

impl<'a> sval::ValueSource<'a, Buffer<'a>> for Buffer<'a> {
    type Error = sval::source::Impossible;

    #[inline]
    fn take(&mut self) -> sval::Result<&Buffer<'a>, sval::source::TakeError<Self::Error>> {
        Ok(self)
    }
}

enum Token<'a> {
    Unstructured(String),
    Str(Cow<'a, str>),
    Text(Cow<'a, Text>),
    Bytes(Cow<'a, Bytes>),
    Null,
    Tag(Tag<Cow<'static, str>>),
    MapBegin(Option<u64>),
    MapEnd,
    MapKeyBegin,
    MapKeyEnd,
    MapValueBegin,
    MapValueEnd,
    SeqBegin(Option<u64>),
    SeqEnd,
    SeqElemBegin,
    SeqElemEnd,
}

impl<'a, 'b> sval::Source<'a> for &'b Token<'a> {
    fn stream_resume<'c, R: sval::Receiver<'c>>(
        &mut self,
        mut receiver: R,
    ) -> sval::Result<sval::source::Stream>
    where
        'a: 'c,
    {
        match *self {
            Token::Str(Cow::Borrowed(value)) => receiver.str(*value)?,
            Token::Str(Cow::Owned(value)) => receiver.str(sval::for_all(value))?,
            Token::Text(Cow::Borrowed(text)) => receiver.text(*text)?,
            Token::Text(Cow::Owned(text)) => receiver.text(sval::for_all(text))?,
            Token::Bytes(Cow::Borrowed(bytes)) => receiver.bytes(*bytes)?,
            Token::Bytes(Cow::Owned(bytes)) => receiver.bytes(sval::for_all(bytes))?,
            Token::Tag(tag) => receiver.tag(tag.by_ref())?,
            Token::Unstructured(value) => receiver.unstructured(value)?,
            Token::Null => receiver.null()?,
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

        Ok(sval::source::Stream::Done)
    }
}
