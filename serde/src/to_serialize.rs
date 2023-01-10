use core::mem;

use serde::ser::{
    Error as _, Serialize as _, SerializeMap as _, SerializeSeq as _, SerializeStruct as _,
    SerializeStructVariant as _, SerializeTuple as _, SerializeTupleStruct as _,
    SerializeTupleVariant as _, Serializer as _,
};

use sval_buffer::{BinaryBuf, TextBuf, ValueBuf};

pub fn to_serialize<V: sval::Value>(value: V) -> ToSerialize<V> {
    ToSerialize(value)
}

pub struct ToSerialize<V>(V);

impl<V: sval::Value> serde::Serialize for ToSerialize<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut stream = Serializer {
            buffered: None,
            state: State::Any(Some(Any {
                serializer,
                struct_label: None,
                variant_label: None,
                variant_index: None,
            })),
        };

        let _ = self.0.stream(&mut stream);

        stream.finish()
    }
}

struct Serializer<'sval, S: serde::Serializer> {
    buffered: Option<Buffered<'sval>>,
    state: State<S>,
}

impl<'sval, S: serde::Serializer> sval::Stream<'sval> for Serializer<'sval, S> {
    fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> sval::Result {
        self.buffer_or_value(|buf| buf.value(value), || ToSerialize(value))
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> sval::Result {
        self.buffer_or_value(|buf| buf.value_computed(value), || ToSerialize(value))
    }

    fn null(&mut self) -> sval::Result {
        self.buffer_or_value(|buf| buf.null(), || None::<()>)
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.buffer_or_value(|buf| buf.bool(value), || value)
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.buffer_or_value(|buf| buf.u8(value), || value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.buffer_or_value(|buf| buf.u16(value), || value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.buffer_or_value(|buf| buf.u32(value), || value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.buffer_or_value(|buf| buf.u64(value), || value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.buffer_or_value(|buf| buf.u128(value), || value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.buffer_or_value(|buf| buf.i8(value), || value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.buffer_or_value(|buf| buf.i16(value), || value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.buffer_or_value(|buf| buf.i32(value), || value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.buffer_or_value(|buf| buf.i64(value), || value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.buffer_or_value(|buf| buf.i128(value), || value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.buffer_or_value(|buf| buf.f32(value), || value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.buffer_or_value(|buf| buf.f64(value), || value)
    }

    fn text_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.text_begin(size_hint),
            |serializer| serializer.put_buffer(Buffered::Text(TextBuf::new())),
        )
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.text_fragment(fragment),
            |serializer| serializer.with_text(|text| text.push_fragment(fragment)),
        )
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.text_fragment_computed(fragment),
            |serializer| serializer.with_text(|text| text.push_fragment_computed(fragment)),
        )
    }

    fn text_end(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.text_end(),
            |serializer| {
                let buf = serializer.take_text()?;

                serializer.state.serialize_value(buf.get())
            },
        )
    }

    fn binary_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.binary_begin(size_hint),
            |serializer| serializer.put_buffer(Buffered::Binary(BinaryBuf::new())),
        )
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.binary_fragment(fragment),
            |serializer| serializer.with_binary(|binary| binary.push_fragment(fragment)),
        )
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.binary_fragment_computed(fragment),
            |serializer| serializer.with_binary(|binary| binary.push_fragment_computed(fragment)),
        )
    }

    fn binary_end(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.binary_end(),
            |serializer| {
                let buf = serializer.take_binary()?;

                serializer.state.serialize_value(Bytes(buf.get()))
            },
        )
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.buffer_or_transition_any_with(
            |buf| buf.map_begin(num_entries_hint),
            |serializer| {
                Ok(State::Map(Some(Map {
                    serializer: serializer.serializer.serialize_map(num_entries_hint)?,
                    is_key: true,
                })))
            },
        )
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.map_key_begin(),
            |serializer| {
                serializer.with_map(|serializer| {
                    serializer.is_key = true;

                    Ok(())
                })
            },
        )
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(|buf| buf.map_key_end(), |_| Ok(()))
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.map_value_begin(),
            |serializer| {
                serializer.with_map(|serializer| {
                    serializer.is_key = false;

                    Ok(())
                })
            },
        )
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(|buf| buf.map_value_end(), |_| Ok(()))
    }

    fn map_end(&mut self) -> sval::Result {
        self.buffer_or_transition_done_with(
            |buf| buf.map_end(),
            |serializer| serializer.take_map()?.serializer.end(),
        )
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.buffer_or_transition_any_with(
            |buf| buf.seq_begin(num_entries_hint),
            |serializer| {
                Ok(State::Seq(Some(Seq {
                    serializer: serializer.serializer.serialize_seq(num_entries_hint)?,
                })))
            },
        )
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(|buf| buf.seq_value_begin(), |_| Ok(()))
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.buffer_or_serialize_with(|buf| buf.seq_value_begin(), |_| Ok(()))
    }

    fn seq_end(&mut self) -> sval::Result {
        self.buffer_or_transition_done_with(
            |buf| buf.seq_end(),
            |serializer| serializer.take_seq()?.serializer.end(),
        )
    }

    fn enum_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        // newtype variant (same as tagged)
        todo!()
    }

    fn enum_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tagged_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        // newtype variant
        todo!()
    }

    fn tagged_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tag(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        // unit variant

        /*
        self.buffer_or_serialize_with(
            |buf| buf.tag(tag, label, index),
            |stream| match tag {
                Some(sval::tags::RUST_OPTION_NONE) => stream.serialize_value(None::<()>),
                Some(sval::tags::RUST_UNIT) => stream.serialize_value(()),
                _ => todo!(),
            },
        )
        */

        todo!()
    }

    fn record_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.buffer_or_transition_any_with(
            |buf| buf.record_begin(tag, label, index, num_entries),
            |serializer| {
                let struct_label = label
                    .and_then(|label| label.try_get_static())
                    .ok_or_else(|| S::Error::custom("missing struct label"))?;

                let num_entries =
                    num_entries.ok_or_else(|| S::Error::custom("missing struct field count"))?;

                Ok(State::Struct(Some(Struct {
                    serializer: serializer
                        .serializer
                        .serialize_struct(struct_label, num_entries)?,
                    label: None,
                })))
            },
        )
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.record_value_begin(label),
            |serializer| {
                serializer.with_struct(|serializer| {
                    serializer.label = label.try_get_static();

                    Ok(())
                })
            },
        )
    }

    fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
        self.buffer_or_serialize_with(
            |buf| buf.record_value_end(label),
            |serializer| {
                serializer.with_struct(|serializer| {
                    serializer.label = None;

                    Ok(())
                })
            },
        )
    }

    fn record_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        self.buffer_or_transition_done_with(
            |buf| buf.record_end(tag, label, index),
            |serializer| serializer.take_struct()?.serializer.end(),
        )
    }

    fn tuple_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.buffer_or_transition_any_with(
            |buf| buf.tuple_begin(tag, label, index, num_entries),
            |serializer| {
                let struct_label = label.and_then(|label| label.try_get_static());

                let num_entries =
                    num_entries.ok_or_else(|| S::Error::custom("missing struct field count"))?;

                if let Some(struct_label) = struct_label {
                    Ok(State::TupleStruct(Some(TupleStruct {
                        serializer: serializer
                            .serializer
                            .serialize_tuple_struct(struct_label, num_entries)?,
                    })))
                } else {
                    Ok(State::Tuple(Some(Tuple {
                        serializer: serializer.serializer.serialize_tuple(num_entries)?,
                    })))
                }
            },
        )
    }

    fn tuple_value_begin(&mut self, index: sval::Index) -> sval::Result {
        self.buffer_or_serialize_with(|buf| buf.tuple_value_begin(index), |_| Ok(()))
    }

    fn tuple_value_end(&mut self, index: sval::Index) -> sval::Result {
        self.buffer_or_serialize_with(|buf| buf.tuple_value_end(index), |_| Ok(()))
    }

    fn tuple_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        self.buffer_or_transition_done_with(
            |buf| buf.tuple_end(tag, label, index),
            |serializer| match serializer.take_tuple()? {
                TakeTuple::Tuple(serializer) => serializer.serializer.end(),
                TakeTuple::TupleStruct(serializer) => serializer.serializer.end(),
            },
        )
    }
}

impl<S: serde::Serializer> State<S> {
    fn serialize_value<T: serde::Serialize>(&mut self, v: T) -> sval::Result {
        let mut r = || match self {
            State::Any(serializer) => {
                match v.serialize(
                    serializer
                        .take()
                        .ok_or_else(|| S::Error::custom("missing serializer"))?
                        .serializer,
                ) {
                    Ok(r) => Ok(Some(r)),
                    Err(e) => Err(e),
                }
            }
            State::Map(serializer) => {
                let serializer = serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?;

                if serializer.is_key {
                    serializer.serializer.serialize_key(&v)?;

                    Ok(None)
                } else {
                    serializer.serializer.serialize_value(&v)?;

                    Ok(None)
                }
            }
            State::Seq(serializer) => {
                serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?
                    .serializer
                    .serialize_element(&v)?;

                Ok(None)
            }
            State::Struct(serializer) => {
                let serializer = serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?;

                serializer.serializer.serialize_field(
                    serializer
                        .label
                        .ok_or_else(|| S::Error::custom("missing field label"))?,
                    &v,
                )?;

                Ok(None)
            }
            State::StructVariant(serializer) => {
                let serializer = serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?;

                serializer.serializer.serialize_field(
                    serializer
                        .label
                        .ok_or_else(|| S::Error::custom("missing field label"))?,
                    &v,
                )?;

                Ok(None)
            }
            State::Tuple(serializer) => {
                serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?
                    .serializer
                    .serialize_element(&v)?;

                Ok(None)
            }
            State::TupleStruct(serializer) => {
                serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?
                    .serializer
                    .serialize_field(&v)?;

                Ok(None)
            }
            State::TupleVariant(serializer) => {
                serializer
                    .as_mut()
                    .ok_or_else(|| S::Error::custom("missing serializer"))?
                    .serializer
                    .serialize_field(&v)?;

                Ok(None)
            }
            State::Done(_) => Err(S::Error::custom("already completed")),
        };

        match r() {
            Ok(Some(r)) => {
                *self = State::Done(Ok(r));
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => {
                *self = State::Done(Err(e));
                Err(sval::Error::unsupported())
            }
        }
    }
}

fn try_catch<'sval, T, S: serde::Serializer>(
    serializer: &mut Serializer<'sval, S>,
    f: impl FnOnce(&mut Serializer<'sval, S>) -> Result<T, S::Error>,
) -> sval::Result<T> {
    match f(serializer) {
        Ok(v) => Ok(v),
        Err(e) => {
            serializer.state = State::Done(Err(e));

            sval::result::unsupported()
        }
    }
}

impl<'sval, S: serde::Serializer> Serializer<'sval, S> {
    fn buffer_or_serialize_with(
        &mut self,
        buffer: impl FnOnce(&mut sval_buffer::ValueBuf<'sval>) -> sval::Result,
        stream: impl FnOnce(&mut Self) -> sval::Result,
    ) -> sval::Result {
        match self {
            Serializer {
                buffered: Some(Buffered::Value(ref mut buf)),
                ..
            } => buffer(buf),
            serializer => stream(serializer),
        }
    }

    fn buffer_or_value<T: serde::Serialize>(
        &mut self,
        buffer: impl FnOnce(&mut sval_buffer::ValueBuf<'sval>) -> sval::Result,
        value: impl FnOnce() -> T,
    ) -> sval::Result {
        self.buffer_or_serialize_with(buffer, |stream| stream.state.serialize_value(value()))
    }

    fn put_buffer(&mut self, buf: Buffered<'sval>) -> sval::Result {
        try_catch(self, |serializer| match serializer.buffered {
            None => {
                serializer.buffered = Some(buf);

                Ok(())
            }
            Some(_) => Err(S::Error::custom("a buffer is already active")),
        })
    }

    fn buffer_or_transition_any_with(
        &mut self,
        mut buffer: impl FnMut(&mut sval_buffer::ValueBuf<'sval>) -> sval::Result,
        transition: impl FnOnce(Any<S>) -> Result<State<S>, S::Error>,
    ) -> sval::Result {
        let buf = try_catch(self, |serializer| {
            match serializer {
                Serializer {
                    buffered: Some(Buffered::Value(ref mut buf)),
                    ..
                } => {
                    buffer(buf).map_err(|_| S::Error::custom("failed to buffer a value"))?;

                    return Ok(None);
                }
                Serializer {
                    buffered: None,
                    state: State::Any(any),
                } => {
                    if let Ok(state) = transition(
                        any.take()
                            .ok_or_else(|| S::Error::custom("missing serializer"))?,
                    ) {
                        serializer.state = state;

                        return Ok(None);
                    }
                }
                _ => return Err(S::Error::custom("invalid serializer state")),
            }

            let mut value = ValueBuf::new();
            buffer(&mut value).map_err(|_| S::Error::custom("failed to buffer a value"))?;

            Ok(Some(Buffered::Value(value)))
        })?;

        self.buffered = buf;

        Ok(())
    }

    fn buffer_or_transition_done_with(
        &mut self,
        buffer: impl FnOnce(&mut sval_buffer::ValueBuf<'sval>) -> sval::Result,
        transition: impl FnOnce(&mut Serializer<S>) -> Result<S::Ok, S::Error>,
    ) -> sval::Result {
        let r = try_catch(self, |serializer| match serializer {
            Serializer {
                buffered: Some(Buffered::Value(ref mut buf)),
                ..
            } => {
                buffer(buf).map_err(|_| S::Error::custom("failed to buffer a value"))?;

                if buf.is_complete() {
                    // Errors handled internally by `serialize_value`
                    let _ = serializer.state.serialize_value(ToSerialize(&*buf));
                    serializer.buffered = None;
                }

                return Ok(None);
            }
            Serializer { buffered: None, .. } => Ok(Some(transition(serializer)?)),
            _ => return Err(S::Error::custom("invalid serializer state")),
        })?;

        if let Some(r) = r {
            self.state = State::Done(Ok(r));
        }

        Ok(())
    }

    fn with_map(&mut self, f: impl FnOnce(&mut Map<S>) -> Result<(), S::Error>) -> sval::Result {
        try_catch(self, |serializer| match serializer {
            Serializer {
                buffered: None,
                state: State::Map(Some(map)),
            } => f(map),
            _ => Err(S::Error::custom("invalid serializer state")),
        })
    }

    fn take_map(&mut self) -> Result<Map<S>, S::Error> {
        match self {
            Serializer {
                buffered: None,
                state: State::Map(map),
            } => map
                .take()
                .ok_or_else(|| S::Error::custom("invalid serializer state")),
            _ => Err(S::Error::custom("invalid serializer state")),
        }
    }

    fn with_seq(&mut self, f: impl FnOnce(&mut Seq<S>) -> Result<(), S::Error>) -> sval::Result {
        try_catch(self, |serializer| match serializer {
            Serializer {
                buffered: None,
                state: State::Seq(Some(seq)),
            } => f(seq),
            _ => Err(S::Error::custom("invalid serializer state")),
        })
    }

    fn take_seq(&mut self) -> Result<Seq<S>, S::Error> {
        match self {
            Serializer {
                buffered: None,
                state: State::Seq(seq),
            } => seq
                .take()
                .ok_or_else(|| S::Error::custom("invalid serializer state")),
            _ => Err(S::Error::custom("invalid serializer state")),
        }
    }

    fn with_struct(
        &mut self,
        f: impl FnOnce(&mut Struct<S>) -> Result<(), S::Error>,
    ) -> sval::Result {
        try_catch(self, |serializer| match serializer {
            Serializer {
                buffered: None,
                state: State::Struct(Some(s)),
            } => f(s),
            _ => Err(S::Error::custom("invalid serializer state")),
        })
    }

    fn take_struct(&mut self) -> Result<Struct<S>, S::Error> {
        match self {
            Serializer {
                buffered: None,
                state: State::Struct(s),
            } => s
                .take()
                .ok_or_else(|| S::Error::custom("invalid serializer state")),
            _ => Err(S::Error::custom("invalid serializer state")),
        }
    }

    fn take_tuple(&mut self) -> Result<TakeTuple<S>, S::Error> {
        match self {
            Serializer {
                buffered: None,
                state: State::Tuple(s),
            } => {
                Ok(TakeTuple::Tuple(s.take().ok_or_else(|| {
                    S::Error::custom("invalid serializer state")
                })?))
            }
            Serializer {
                buffered: None,
                state: State::TupleStruct(s),
            } => {
                Ok(TakeTuple::TupleStruct(s.take().ok_or_else(|| {
                    S::Error::custom("invalid serializer state")
                })?))
            }
            _ => Err(S::Error::custom("invalid serializer state")),
        }
    }

    fn with_text(&mut self, f: impl FnOnce(&mut TextBuf<'sval>) -> sval::Result) -> sval::Result {
        try_catch(self, |serializer| match serializer.buffered {
            Some(Buffered::Text(ref mut buf)) => {
                f(buf).map_err(|_| S::Error::custom("failed to buffer a text fragment"))
            }
            _ => Err(S::Error::custom("no active text buffer")),
        })
    }

    fn take_text(&mut self) -> sval::Result<TextBuf<'sval>> {
        try_catch(self, |serializer| match serializer.buffered {
            Some(Buffered::Text(ref mut buf)) => {
                let buf = mem::take(buf);
                serializer.buffered = None;

                Ok(buf)
            }
            _ => Err(S::Error::custom("no active text buffer")),
        })
    }

    fn with_binary(
        &mut self,
        f: impl FnOnce(&mut BinaryBuf<'sval>) -> sval::Result,
    ) -> sval::Result {
        try_catch(self, |serializer| match serializer.buffered {
            Some(Buffered::Binary(ref mut buf)) => {
                f(buf).map_err(|_| S::Error::custom("failed to buffer a binary fragment"))
            }
            _ => Err(S::Error::custom("no active binary buffer")),
        })
    }

    fn take_binary(&mut self) -> sval::Result<BinaryBuf<'sval>> {
        try_catch(self, |serializer| match serializer.buffered {
            Some(Buffered::Binary(ref mut buf)) => {
                let buf = mem::take(buf);
                serializer.buffered = None;

                Ok(buf)
            }
            _ => Err(S::Error::custom("no active binary buffer")),
        })
    }

    fn finish(self) -> Result<S::Ok, S::Error> {
        if let State::Done(r) = self.state {
            r
        } else {
            panic!("incomplete serializer")
        }
    }
}

enum Buffered<'sval> {
    Text(TextBuf<'sval>),
    Binary(BinaryBuf<'sval>),
    Value(ValueBuf<'sval>),
}

enum State<S: serde::Serializer> {
    Any(Option<Any<S>>),
    Map(Option<Map<S>>),
    Seq(Option<Seq<S>>),
    Struct(Option<Struct<S>>),
    StructVariant(Option<StructVariant<S>>),
    Tuple(Option<Tuple<S>>),
    TupleStruct(Option<TupleStruct<S>>),
    TupleVariant(Option<TupleVariant<S>>),
    Done(Result<S::Ok, S::Error>),
}

struct Any<S: serde::Serializer> {
    serializer: S,
    struct_label: Option<&'static str>,
    variant_label: Option<&'static str>,
    variant_index: Option<u32>,
}

struct Map<S: serde::Serializer> {
    serializer: S::SerializeMap,
    is_key: bool,
}

struct Seq<S: serde::Serializer> {
    serializer: S::SerializeSeq,
}

struct Struct<S: serde::Serializer> {
    serializer: S::SerializeStruct,
    label: Option<&'static str>,
}
struct StructVariant<S: serde::Serializer> {
    serializer: S::SerializeStructVariant,
    label: Option<&'static str>,
}
struct Tuple<S: serde::Serializer> {
    serializer: S::SerializeTuple,
}
struct TupleStruct<S: serde::Serializer> {
    serializer: S::SerializeTupleStruct,
}
struct TupleVariant<S: serde::Serializer> {
    serializer: S::SerializeTupleVariant,
}

enum TakeTuple<S: serde::Serializer> {
    Tuple(Tuple<S>),
    TupleStruct(TupleStruct<S>),
}

struct Bytes<'sval>(&'sval [u8]);

impl<'sval> serde::Serialize for Bytes<'sval> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.0)
    }
}
