use serde::ser::{
    Error as _, Serialize as _, SerializeMap as _, SerializeSeq as _, SerializeStruct as _,
    SerializeStructVariant as _, SerializeTuple as _, SerializeTupleStruct as _,
    SerializeTupleVariant as _, Serializer as _,
};

pub fn to_serialize<V: sval::Value>(value: V) -> ToSerialize<V> {
    ToSerialize(value)
}

pub struct ToSerialize<V>(V);

impl<V: sval::Value> serde::Serialize for ToSerialize<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut stream = Serializer {
            buffered: None,
            state: State::Any {
                serializer: Some(serializer),
                struct_label: None,
                variant_label: None,
                variant_index: None,
            },
        };

        let _ = self.0.stream(&mut stream);

        stream.finish()
    }
}

struct Serializer<'sval, S: serde::Serializer> {
    buffered: Option<Buffered<'sval>>,
    state: State<S>,
}

enum Buffered<'sval> {
    Text(sval_buffer::TextBuf<'sval>),
    Binary(sval_buffer::BinaryBuf<'sval>),
    Value(sval_buffer::ValueBuf<'sval>),
}

struct Bytes<'sval>(&'sval [u8]);

impl<'sval> serde::Serialize for Bytes<'sval> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.0)
    }
}

enum State<S: serde::Serializer> {
    Any {
        serializer: Option<S>,
        struct_label: Option<&'static str>,
        variant_label: Option<&'static str>,
        variant_index: Option<u32>,
    },
    Map {
        serializer: Option<S::SerializeMap>,
        is_key: bool,
    },
    Seq {
        serializer: Option<S::SerializeSeq>,
    },
    Struct {
        serializer: Option<S::SerializeStruct>,
        label: Option<&'static str>,
    },
    StructVariant {
        serializer: Option<S::SerializeStructVariant>,
        label: Option<&'static str>,
    },
    Tuple {
        serializer: Option<S::SerializeTuple>,
    },
    TupleStruct {
        serializer: Option<S::SerializeTupleStruct>,
    },
    TupleVariant {
        serializer: Option<S::SerializeTupleVariant>,
    },
    Done(Result<S::Ok, S::Error>),
}

impl<'sval, S: serde::Serializer> Serializer<'sval, S> {
    fn serialize_value(&mut self, v: impl serde::Serialize) -> sval::Result {
        match &mut self.state {
            State::Any { serializer, .. } => {
                match v.serialize(serializer.take().expect("missing serializer")) {
                    Ok(r) => {
                        self.state = State::Done(Ok(r));
                        Ok(())
                    }
                    Err(e) => {
                        self.state = State::Done(Err(e));
                        Err(sval::Error::unsupported())
                    }
                }
            }
            State::Map {
                serializer,
                is_key: true,
            } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_key(&v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::Map {
                serializer,
                is_key: false,
            } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_value(&v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::Seq { serializer } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_element(&v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::Struct { serializer, label } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_field(label.expect("missing field label"), &v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::StructVariant { serializer, label } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_field(label.expect("missing field label"), &v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::Tuple { serializer } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_element(&v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::TupleStruct { serializer } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_field(&v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::TupleVariant { serializer } => match serializer
                .as_mut()
                .expect("missing serializer")
                .serialize_field(&v)
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            State::Done(_) => panic!("serializer is already complete"),
        }
    }

    fn buffer_text_begin(&mut self) -> sval::Result {
        self.buffered = Some(Buffered::Text(sval_buffer::TextBuf::new()));
        Ok(())
    }

    fn buffer_text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        match self.buffered {
            Some(Buffered::Text(ref mut text)) => match text.push_fragment(fragment) {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(S::Error::custom(
                        "cannot buffer multiple text fragments",
                    )));

                    Err(e)
                }
            },
            _ => {
                self.state = State::Done(Err(S::Error::custom("unexpected buffer state")));

                Err(sval::Error::unsupported())
            }
        }
    }

    fn buffer_text_fragment_computed(&mut self, _: &str) -> sval::Result {
        self.state = State::Done(Err(S::Error::custom(
            "cannot buffer computed text fragments",
        )));

        Err(sval::Error::unsupported())
    }

    fn buffer_binary_begin(&mut self) -> sval::Result {
        self.buffered = Some(Buffered::Binary(sval_buffer::BinaryBuf::new()));
        Ok(())
    }

    fn buffer_binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        match self.buffered {
            Some(Buffered::Binary(ref mut binary)) => match binary.push_fragment(fragment) {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.state = State::Done(Err(S::Error::custom(
                        "cannot buffer multiple binary fragments",
                    )));

                    Err(e)
                }
            },
            _ => {
                self.state = State::Done(Err(S::Error::custom("unexpected buffer state")));

                Err(sval::Error::unsupported())
            }
        }
    }

    fn buffer_binary_fragment_computed(&mut self, _: &[u8]) -> sval::Result {
        self.state = State::Done(Err(S::Error::custom(
            "cannot buffer computed binary fragments",
        )));

        Err(sval::Error::unsupported())
    }

    fn buffer_end(&mut self) -> sval::Result {
        match self.buffered.take() {
            Some(Buffered::Text(v)) => self.serialize_value(v.get()),
            Some(Buffered::Binary(v)) => self.serialize_value(Bytes(v.get())),
            None => panic!("missing buffered value"),
        }
    }

    fn serialize_map_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        match self.state {
            State::Any {
                ref mut serializer, ..
            } => {
                match serializer
                    .take()
                    .expect("missing serializer")
                    .serialize_map(size_hint)
                {
                    Ok(serializer) => {
                        self.state = State::Map {
                            serializer: Some(serializer),
                            is_key: true,
                        };

                        Ok(())
                    }
                    Err(e) => {
                        self.state = State::Done(Err(e));

                        Err(sval::Error::unsupported())
                    }
                }
            }
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_map_end(&mut self) -> sval::Result {
        match self.state {
            State::Map {
                ref mut serializer, ..
            } => match serializer.take().expect("missing serializer").end() {
                Ok(r) => {
                    self.state = State::Done(Ok(r));
                    Ok(())
                }
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_map_key(&mut self) -> sval::Result {
        match self.state {
            State::Map { ref mut is_key, .. } => {
                *is_key = true;

                Ok(())
            }
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_map_value(&mut self) -> sval::Result {
        match self.state {
            State::Map { ref mut is_key, .. } => {
                *is_key = false;

                Ok(())
            }
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_seq_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        match self.state {
            State::Any {
                ref mut serializer, ..
            } => {
                match serializer
                    .take()
                    .expect("missing serializer")
                    .serialize_seq(size_hint)
                {
                    Ok(serializer) => {
                        self.state = State::Seq {
                            serializer: Some(serializer),
                        };

                        Ok(())
                    }
                    Err(e) => {
                        self.state = State::Done(Err(e));

                        Err(sval::Error::unsupported())
                    }
                }
            }
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_seq_end(&mut self) -> sval::Result {
        match self.state {
            State::Seq { ref mut serializer } => {
                match serializer.take().expect("missing serializer").end() {
                    Ok(r) => {
                        self.state = State::Done(Ok(r));
                        Ok(())
                    }
                    Err(e) => {
                        self.state = State::Done(Err(e));
                        Err(sval::Error::unsupported())
                    }
                }
            }
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_struct_begin(&mut self, size: usize) -> sval::Result {
        match self.state {
            State::Any {
                ref mut serializer,
                struct_label: Some(struct_label),
                ..
            } => {
                match serializer
                    .take()
                    .expect("missing serializer")
                    .serialize_struct(struct_label, size)
                {
                    Ok(serializer) => {
                        self.state = State::Struct {
                            serializer: Some(serializer),
                            label: None,
                        };

                        Ok(())
                    }
                    Err(e) => {
                        self.state = State::Done(Err(e));

                        Err(sval::Error::unsupported())
                    }
                }
            }
            _ => panic!("unexpected serializer state"),
        }
    }

    fn serialize_struct_end(&mut self) -> sval::Result {
        match self.state {
            State::Map {
                ref mut serializer, ..
            } => match serializer.take().expect("missing serializer").end() {
                Ok(r) => {
                    self.state = State::Done(Ok(r));
                    Ok(())
                }
                Err(e) => {
                    self.state = State::Done(Err(e));
                    Err(sval::Error::unsupported())
                }
            },
            _ => panic!("unexpected serializer state"),
        }
    }

    fn finish(self) -> Result<S::Ok, S::Error> {
        if let State::Done(r) = self.state {
            r
        } else {
            panic!("incomplete serializer")
        }
    }
}

impl<'sval, S: serde::Serializer> sval::Stream<'sval> for Serializer<'sval, S> {
    fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> sval::Result {
        self.value_computed(value)
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> sval::Result {
        self.serialize_value(ToSerialize(value))
    }

    fn null(&mut self) -> sval::Result {
        self.serialize_value(None::<()>)
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.serialize_value(value)
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.buffer_text_begin()
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.buffer_text_fragment(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.buffer_text_fragment_computed(fragment)
    }

    fn text_end(&mut self) -> sval::Result {
        self.buffer_end()
    }

    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.buffer_binary_begin()
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.buffer_binary_fragment(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.buffer_binary_fragment_computed(fragment)
    }

    fn binary_end(&mut self) -> sval::Result {
        self.buffer_end()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.serialize_value(value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.serialize_value(value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.serialize_value(value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.serialize_value(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.serialize_value(value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.serialize_value(value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.serialize_value(value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.serialize_value(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.serialize_value(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.serialize_value(value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.serialize_value(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.serialize_value(value)
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.serialize_map_begin(num_entries_hint)
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.serialize_map_key()
    }

    fn map_key_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.serialize_map_value()
    }

    fn map_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.serialize_map_end()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.serialize_seq_begin(num_entries_hint)
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.serialize_seq_end()
    }

    fn enum_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
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
        match tag {
            Some(sval::tags::RUST_OPTION_NONE) => self.serialize_value(None::<()>),
            Some(sval::tags::RUST_UNIT) => self.serialize_value(()),
            _ => todo!(),
        }
    }

    fn record_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        todo!()
    }

    fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
        todo!()
    }

    fn record_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_value_begin(&mut self, index: sval::Index) -> sval::Result {
        todo!()
    }

    fn tuple_value_end(&mut self, index: sval::Index) -> sval::Result {
        todo!()
    }

    fn tuple_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }
}
