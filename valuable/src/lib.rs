use valuable::{Listable, Mappable, Valuable, Visit};

use sval_generic_api::{receiver, source, value, Receiver, Result};

use sval_generic_api_buffer::{buffer, BufferReceiver};

pub struct Value<'a, V>(Detected<'a, V>, &'a V);

impl<'a, V: value::Value> Value<'a, V> {
    pub fn new(value: &'a V) -> Self {
        Value(Detected::detect(value), value)
    }
}

pub fn value<V: value::Value>(value: &V) -> Value<V> {
    Value::new(value)
}

enum Detected<'a, V> {
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

impl<'a, V: value::Value> Detected<'a, V> {
    fn detect(value: &'a V) -> Self {
        struct Detect<'a, V>(Detected<'a, V>, &'a V);

        impl<'a, V> Receiver<'a> for Detect<'a, V> {
            fn display<D: receiver::Display>(&mut self, _: D) -> Result {
                self.0 = Detected::Unknown;

                receiver::unsupported()
            }

            fn u64(&mut self, value: u64) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::Unsigned(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn i64(&mut self, value: i64) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::Signed(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn u128(&mut self, value: u128) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::BigUnsigned(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn i128(&mut self, value: i128) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::BigSigned(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn f64(&mut self, value: f64) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::Float(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn bool(&mut self, value: bool) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::Bool(value));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn none(&mut self) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::Unit);

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn str<'s: 'a, S: source::ValueSource<'s, str>>(&mut self, mut value: S) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Primitive(Primitive::Str(value.take_ref()?));

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn map_begin(&mut self, len: Option<usize>) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Map(Map { len, map: self.1 });

                    Ok(())
                } else {
                    receiver::unsupported()
                }
            }

            fn seq_begin(&mut self, len: Option<usize>) -> Result {
                if let Detected::Unknown = self.0 {
                    self.0 = Detected::Sequence(Sequence { len, seq: self.1 });

                    Ok(())
                } else {
                    receiver::unsupported()
                }
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

        let mut detected = Detect(Detected::Unknown, value);
        let _ = value.stream(&mut detected);

        detected.0
    }

    fn as_value(&self) -> valuable::Value<'_> {
        match self {
            Detected::Unknown => unreachable!(),
            Detected::Primitive(Primitive::Bool(value)) => valuable::Value::Bool(*value),
            Detected::Primitive(Primitive::Signed(value)) => valuable::Value::I64(*value),
            Detected::Primitive(Primitive::Unsigned(value)) => valuable::Value::U64(*value),
            Detected::Primitive(Primitive::BigSigned(value)) => valuable::Value::I128(*value),
            Detected::Primitive(Primitive::BigUnsigned(value)) => valuable::Value::U128(*value),
            Detected::Primitive(Primitive::Float(value)) => valuable::Value::F64(*value),
            Detected::Primitive(Primitive::Str(value)) => valuable::Value::String(value),
            Detected::Primitive(Primitive::Unit) => valuable::Value::Unit,
            Detected::Map(map) => valuable::Value::Mappable(map),
            Detected::Sequence(seq) => valuable::Value::Listable(seq),
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

struct ValuableReceiver<'a>(&'a mut dyn Visit);

impl<'a, 'b> Receiver<'a> for ValuableReceiver<'b> {
    fn display<D: receiver::Display>(&mut self, _: D) -> Result {
        receiver::unsupported()
    }

    fn u64(&mut self, value: u64) -> Result {
        self.0.visit_value(valuable::Value::U64(value));

        Ok(())
    }

    fn i64(&mut self, value: i64) -> Result {
        self.0.visit_value(valuable::Value::I64(value));

        Ok(())
    }

    fn u128(&mut self, value: u128) -> Result {
        self.0.visit_value(valuable::Value::U128(value));

        Ok(())
    }

    fn i128(&mut self, value: i128) -> Result {
        self.0.visit_value(valuable::Value::I128(value));

        Ok(())
    }

    fn f64(&mut self, value: f64) -> Result {
        self.0.visit_value(valuable::Value::F64(value));

        Ok(())
    }

    fn bool(&mut self, value: bool) -> Result {
        self.0.visit_value(valuable::Value::Bool(value));

        Ok(())
    }

    fn none(&mut self) -> Result {
        self.0.visit_value(valuable::Value::Unit);

        Ok(())
    }

    fn str<'s: 'a, S: source::ValueSource<'s, str>>(&mut self, mut value: S) -> Result {
        self.0.visit_value(valuable::Value::String(value.take()?));

        Ok(())
    }

    fn map_entry<'k: 'a, 'v: 'a, K: source::Source<'k>, V: source::Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        // In order to visit an entry we need both the key and value to be available
        struct BufferKey<'a, V>(&'a mut dyn Visit, V);

        impl<'a, 'k, 'v, V: source::Source<'v>> BufferReceiver<'k> for BufferKey<'a, V> {
            fn value_source<
                'b: 'k,
                K: value::Value + ?Sized + 'b,
                S: source::ValueSource<'b, K>,
            >(
                &mut self,
                mut v: S,
            ) -> Result {
                struct BufferValue<'a, 'k, K: ?Sized + 'k>(&'a mut dyn Visit, &'k K);

                impl<'a, 'k, 'v, K: value::Value + ?Sized + 'k> BufferReceiver<'v> for BufferValue<'a, 'k, K> {
                    fn value_source<
                        'b: 'v,
                        V: value::Value + ?Sized + 'b,
                        S: source::ValueSource<'b, V>,
                    >(
                        &mut self,
                        mut v: S,
                    ) -> Result {
                        let key = self.1;
                        let value = v.take()?;

                        self.0.visit_entry(
                            Value::new(&key).as_value(),
                            Value::new(&value).as_value(),
                        );

                        Ok(())
                    }
                }

                let key = v.take()?;

                buffer(BufferValue(self.0, key), &mut self.1)
            }
        }

        buffer(BufferKey(self.0, value), key)
    }

    fn map_field_entry<'v: 'a, F: source::ValueSource<'static, str>, V: source::Source<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result {
        self.map_entry(field, value)
    }

    fn seq_elem<'e: 'a, E: source::Source<'e>>(&mut self, elem: E) -> Result {
        struct BufferElem<'a>(&'a mut dyn Visit);

        impl<'a, 'e> BufferReceiver<'e> for BufferElem<'a> {
            fn value_source<
                'b: 'e,
                E: value::Value + ?Sized + 'b,
                S: source::ValueSource<'b, E>,
            >(
                &mut self,
                mut e: S,
            ) -> Result {
                let elem = e.take()?;

                self.0.visit_value(Value::new(&elem).as_value());

                Ok(())
            }
        }

        buffer(BufferElem(self.0), elem)
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result {
        Ok(())
    }

    fn map_end(&mut self) -> Result {
        Ok(())
    }

    fn map_key_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> Result {
        Ok(())
    }

    fn seq_end(&mut self) -> Result {
        Ok(())
    }

    fn seq_elem_begin(&mut self) -> Result {
        Ok(())
    }

    fn seq_elem_end(&mut self) -> Result {
        Ok(())
    }
}
