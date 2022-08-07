struct Stream<S: serde::Serializer> {
    context: Context<S>,
    ok: Option<S::Ok>,
}

enum Context<S: serde::Serializer> {
    Any {
        serializer: Option<S>,
        enum_label: Option<&'static str>,
        value_label: Option<&'static str>,
        value_variant: Option<u32>,
    },
    Map {
        serializer: S::SerializeMap,
    },
    Seq {
        serializer: S::SerializeSeq,
    },
    Struct {
        serializer: S::SerializeStruct,
    },
    StructVariant {
        serializer: S::SerializeStructVariant,
        field: Option<(&'static str, u32)>,
    },
    Tuple {
        serializer: S::SerializeTuple,
    },
    TupleVariant {
        serializer: S::SerializeTupleVariant,
        field: Option<u32>,
    },
}

impl<S: serde::Serializer> Stream<S> {
    fn serialize_any(&mut self, value: impl serde::Serialize) -> sval::Result {
        self.ok = Some(match self.context {
            Context::Any {
                ref mut serializer,
                enum_label,
                value_label,
                value_variant,
            } => {
                let serializer = serializer.take().ok_or(sval::Error::unsupported())?;

                match (enum_label, value_label, value_variant) {
                    (Some(enum_label), Some(value_label), Some(value_variant)) => serializer
                        .serialize_newtype_variant(enum_label, value_variant, value_label, &value)
                        .map_err(|_| sval::Error::unsupported()),
                    (None, Some(label), _) => serializer
                        .serialize_newtype_struct(label, &value)
                        .map_err(|_| sval::Error::unsupported()),
                    (None, None, None) => value
                        .serialize(serializer)
                        .map_err(|_| sval::Error::unsupported()),
                    _ => return sval::result::unsupported(),
                }
            }
            _ => todo!(),
        }?);

        Ok(())
    }
}

impl<'sval, S: serde::Serializer> sval::Stream<'sval> for Stream<S> {
    fn unit(&mut self) -> sval::Result {
        self.serialize_any(&())
    }

    fn null(&mut self) -> sval::Result {
        todo!()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        todo!()
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        todo!()
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        todo!()
    }

    fn text_end(&mut self) -> sval::Result {
        todo!()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        todo!()
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        todo!()
    }

    fn binary_end(&mut self) -> sval::Result {
        todo!()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        todo!()
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        todo!()
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn map_key_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn map_key_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn map_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_end(&mut self) -> sval::Result {
        todo!()
    }

    fn enum_begin(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn enum_end(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn tagged_begin(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn tagged_end(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn constant_begin(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn constant_end(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn record_begin(
        &mut self,
        label: Option<sval::Label>,
        id: Option<sval::Id>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn record_value_begin(&mut self, label: sval::Label, id: sval::Id) -> sval::Result {
        todo!()
    }

    fn record_value_end(&mut self, label: sval::Label, id: sval::Id) -> sval::Result {
        todo!()
    }

    fn record_end(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn tuple_begin(
        &mut self,
        label: Option<sval::Label>,
        id: Option<sval::Id>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_value_begin(&mut self, id: sval::Id) -> sval::Result {
        todo!()
    }

    fn tuple_value_end(&mut self, id: sval::Id) -> sval::Result {
        todo!()
    }

    fn tuple_end(&mut self, label: Option<sval::Label>, id: Option<sval::Id>) -> sval::Result {
        todo!()
    }

    fn optional_some_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn optional_some_end(&mut self) -> sval::Result {
        todo!()
    }

    fn optional_none(&mut self) -> sval::Result {
        todo!()
    }

    fn constant_size_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn constant_size_end(&mut self) -> sval::Result {
        todo!()
    }

    fn number_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn number_end(&mut self) -> sval::Result {
        todo!()
    }
}
