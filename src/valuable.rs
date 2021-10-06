use std::cell::Cell;

use valuable::{Listable, Mappable, Valuable, Visit};

use crate::{receiver, source, value, Receiver};

pub struct Value<'a, V>(DetectedValue<'a, V>, &'a V);

impl<'a, V: value::Value> Value<'a, V> {
    pub fn new(value: &'a V) -> Self {
        Value(DetectedValue::detect(value), value)
    }
}

pub fn value<V: value::Value>(value: &V) -> Value<V> {
    Value::new(value)
}

enum DetectedValue<'a, V> {
    Unknown,
    Primitive(Primitive<'a>),
    Map(Map<'a, V>),
    Sequence(Sequence<'a, V>),
}

enum Primitive<'a> {
    Bool(bool),
    Signed(i64),
    Unsigned(u64),
    BigSigned(i128),
    BigUnsigned(u128),
    Float(f64),
    Str(&'a str),
    Unit,
}

struct Map<'a, V> {
    len: Option<usize>,
    map: &'a V,
}

struct Sequence<'a, V> {
    len: Option<usize>,
    seq: &'a V,
}

impl<'a, V: value::Value> Valuable for Map<'a, V> {
    fn as_value(&self) -> valuable::Value<'_> {
        valuable::Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        let _ = self.map.stream(ValuableReceiver(visit));
    }
}

impl<'a, V: value::Value> Mappable for Map<'a, V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.len)
    }
}

impl<'a, V: value::Value> Valuable for Sequence<'a, V> {
    fn as_value(&self) -> valuable::Value<'_> {
        valuable::Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        let _ = self.seq.stream(ValuableReceiver(visit));
    }
}

impl<'a, V: value::Value> Listable for Sequence<'a, V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.len)
    }
}

impl<'a, V: value::Value> DetectedValue<'a, V> {
    fn detect(value: &'a V) -> Self {
        struct Detect<'a, V>(DetectedValue<'a, V>, &'a V);

        impl<'a, V> Receiver<'a> for Detect<'a, V> {
            fn display<D: receiver::Display>(&mut self, _: D) -> crate::Result {
                self.0 = DetectedValue::Unknown;

                receiver::unsupported()
            }

            fn u64(&mut self, value: u64) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::Unsigned(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn i64(&mut self, value: i64) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::Signed(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn u128(&mut self, value: u128) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::BigUnsigned(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn i128(&mut self, value: i128) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::BigSigned(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn f64(&mut self, value: f64) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::Float(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn bool(&mut self, value: bool) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::Bool(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn none(&mut self) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::Unit);

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn str<'s: 'a, S: source::ValueSource<'s, str>>(
                &mut self,
                mut value: S,
            ) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Primitive(Primitive::Str(value.value_ref()?));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn map_begin(&mut self, len: Option<usize>) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Map(Map { len, map: self.1 });

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn seq_begin(&mut self, len: Option<usize>) -> crate::Result {
                if let DetectedValue::Unknown = self.0 {
                    self.0 = DetectedValue::Sequence(Sequence { len, seq: self.1 });

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn map_end(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn map_key_begin(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn map_key_end(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn map_value_begin(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn map_value_end(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn seq_end(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn seq_elem_begin(&mut self) -> crate::Result {
                receiver::unsupported()
            }

            fn seq_elem_end(&mut self) -> crate::Result {
                receiver::unsupported()
            }
        }

        let mut detected = Detect(DetectedValue::Unknown, value);
        let _ = value.stream(&mut detected);

        detected.0
    }

    fn as_value(&self) -> valuable::Value<'_> {
        match self {
            DetectedValue::Unknown => unreachable!(),
            DetectedValue::Primitive(Primitive::Bool(value)) => valuable::Value::Bool(*value),
            DetectedValue::Primitive(Primitive::Signed(value)) => valuable::Value::I64(*value),
            DetectedValue::Primitive(Primitive::Unsigned(value)) => valuable::Value::U64(*value),
            DetectedValue::Primitive(Primitive::BigSigned(value)) => valuable::Value::I128(*value),
            DetectedValue::Primitive(Primitive::BigUnsigned(value)) => {
                valuable::Value::U128(*value)
            }
            DetectedValue::Primitive(Primitive::Float(value)) => valuable::Value::F64(*value),
            DetectedValue::Primitive(Primitive::Str(value)) => valuable::Value::String(value),
            DetectedValue::Primitive(Primitive::Unit) => valuable::Value::Unit,
            DetectedValue::Map(map) => valuable::Value::Mappable(map),
            DetectedValue::Sequence(seq) => valuable::Value::Listable(seq),
        }
    }
}

impl<'a, V: value::Value> Valuable for Value<'a, V> {
    fn as_value(&self) -> valuable::Value<'_> {
        self.0.as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        let _ = self.1.stream(ValuableReceiver(visit));
    }
}

struct Source<V>(DetectedSource, Cell<Option<V>>);

enum DetectedSource {
    Unknown,
    Map,
    Sequence,
}

impl<'a, V: source::Source<'a>> Source<V> {
    pub fn new(value: V) -> Source<V> {
        Source(DetectedSource::detect(&value), Cell::new(Some(value)))
    }
}

impl DetectedSource {
    fn detect<'a>(source: &impl source::Source<'a>) -> Self {
        match (source.is_map_hint(), source.is_seq_hint()) {
            (Some(true), Some(false)) | (Some(true), None) => DetectedSource::Map,
            (Some(false), Some(true)) | (None, Some(true)) => DetectedSource::Sequence,
            (Some(false), Some(false)) => unimplemented!("try capture a primitive"),
            _ => DetectedSource::Unknown,
        }
    }
}

impl<'a, V: source::Source<'a>> Valuable for Source<V> {
    fn as_value(&self) -> valuable::Value<'_> {
        match self.0 {
            DetectedSource::Map => valuable::Value::Mappable(self),
            DetectedSource::Sequence => valuable::Value::Listable(self),
            DetectedSource::Unknown => unreachable!(),
        }
    }

    fn visit(&self, visit: &mut dyn Visit) {
        if let Some(mut source) = self.1.take() {
            let _ = source.stream(ValuableReceiver(visit));
        }
    }
}

impl<'a, V: source::Source<'a>> Mappable for Source<V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<'a, V: source::Source<'a>> Listable for Source<V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

struct ValuableReceiver<'v>(&'v mut dyn Visit);

impl<'a, 'v> Receiver<'a> for ValuableReceiver<'v> {
    fn display<D: receiver::Display>(&mut self, _: D) -> crate::Result {
        receiver::unsupported()
    }

    fn u64(&mut self, value: u64) -> crate::Result {
        self.0.visit_value(valuable::Value::U64(value));

        Ok(())
    }

    fn i64(&mut self, value: i64) -> crate::Result {
        self.0.visit_value(valuable::Value::I64(value));

        Ok(())
    }

    fn u128(&mut self, value: u128) -> crate::Result {
        self.0.visit_value(valuable::Value::U128(value));

        Ok(())
    }

    fn i128(&mut self, value: i128) -> crate::Result {
        self.0.visit_value(valuable::Value::I128(value));

        Ok(())
    }

    fn f64(&mut self, value: f64) -> crate::Result {
        self.0.visit_value(valuable::Value::F64(value));

        Ok(())
    }

    fn bool(&mut self, value: bool) -> crate::Result {
        self.0.visit_value(valuable::Value::Bool(value));

        Ok(())
    }

    fn none(&mut self) -> crate::Result {
        self.0.visit_value(valuable::Value::Unit);

        Ok(())
    }

    fn str<'s: 'a, S: source::ValueSource<'s, str>>(&mut self, mut value: S) -> crate::Result {
        self.0.visit_value(valuable::Value::String(value.value()?));

        Ok(())
    }

    fn map_entry<'k: 'a, 'kv: 'a, K: source::Source<'k>, KV: source::Source<'kv>>(
        &mut self,
        key: K,
        value: KV,
    ) -> crate::Result {
        self.0
            .visit_entry(Source::new(key).as_value(), Source::new(value).as_value());

        Ok(())
    }

    fn seq_elem<'e: 'a, E: source::Source<'e>>(&mut self, elem: E) -> crate::Result {
        self.0.visit_unnamed_fields(&[Source::new(elem).as_value()]);

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> crate::Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> crate::Result {
        Ok(())
    }

    fn map_end(&mut self) -> crate::Result {
        Ok(())
    }

    fn map_key_begin(&mut self) -> crate::Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> crate::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> crate::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> crate::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> crate::Result {
        Ok(())
    }

    fn seq_elem_begin(&mut self) -> crate::Result {
        Ok(())
    }

    fn seq_elem_end(&mut self) -> crate::Result {
        Ok(())
    }
}
