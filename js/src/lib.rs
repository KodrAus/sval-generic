use js_sys::{Array, Map};
use wasm_bindgen::prelude::*;

use sval_generic_api::{
    receiver::{self, Receiver},
    source, value,
};

pub fn value<'a>(mut v: impl source::Source<'a>) -> value::Result<JsValue> {
    let mut receiver = JsReceiver::new();
    v.stream_to_end(&mut receiver)?;

    Ok(receiver.into_value())
}

struct JsReceiver {
    target: JsTarget,
}

enum JsTarget {
    Empty,
    Value(JsValue),
    Map { target: Map, key: Option<JsValue> },
    Array { target: Array },
}

impl JsReceiver {
    fn new() -> Self {
        JsReceiver {
            target: JsTarget::Empty,
        }
    }

    fn push(&mut self, v: impl Into<JsValue>) -> receiver::Result {
        match self.target {
            JsTarget::Empty => {
                self.target = JsTarget::Value(v.into());

                Ok(())
            }
            JsTarget::Value(_) => Err(receiver::Error),
            JsTarget::Map {
                ref mut target,
                ref mut key,
            } => match key.take() {
                Some(key) => {
                    target.set(&key, &v.into());

                    Ok(())
                }
                None => {
                    *key = Some(v.into());

                    Ok(())
                }
            },
            JsTarget::Array { ref mut target } => {
                target.push(&v.into());

                Ok(())
            }
        }
    }

    fn map(&mut self) -> receiver::Result {
        match self.target {
            JsTarget::Empty => {
                self.target = JsTarget::Map {
                    target: Map::new(),
                    key: None,
                };

                Ok(())
            }
            _ => Err(receiver::Error),
        }
    }

    fn array(&mut self) -> receiver::Result {
        match self.target {
            JsTarget::Empty => {
                self.target = JsTarget::Array {
                    target: Array::new(),
                };

                Ok(())
            }
            _ => Err(receiver::Error),
        }
    }

    fn into_value(self) -> JsValue {
        match self.target {
            JsTarget::Empty => JsValue::null(),
            JsTarget::Value(value) => value,
            JsTarget::Map { target, .. } => target.into(),
            JsTarget::Array { target, .. } => target.into(),
        }
    }
}

impl<'a> Receiver<'a> for JsReceiver {
    #[inline]
    fn display<D: receiver::Display>(&mut self, v: D) -> receiver::Result {
        self.push(v.to_string())
    }

    #[inline]
    fn none(&mut self) -> receiver::Result {
        self.push(JsValue::null())
    }

    #[inline]
    fn map_begin(&mut self, _: Option<usize>) -> receiver::Result {
        self.map()
    }

    #[inline]
    fn map_end(&mut self) -> receiver::Result {
        Ok(())
    }

    #[inline]
    fn map_key_begin(&mut self) -> receiver::Result {
        Ok(())
    }

    #[inline]
    fn map_key_end(&mut self) -> receiver::Result {
        Ok(())
    }

    #[inline]
    fn map_value_begin(&mut self) -> receiver::Result {
        Ok(())
    }

    #[inline]
    fn map_value_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_key<'k: 'a, K: receiver::Source<'k>>(&mut self, k: K) -> receiver::Result {
        self.push(value(k)?)
    }

    fn map_value<'v: 'a, V: receiver::Source<'v>>(&mut self, v: V) -> receiver::Result {
        self.push(value(v)?)
    }

    #[inline]
    fn seq_begin(&mut self, _: Option<usize>) -> receiver::Result {
        self.array()
    }

    #[inline]
    fn seq_end(&mut self) -> receiver::Result {
        Ok(())
    }

    #[inline]
    fn seq_elem_begin(&mut self) -> receiver::Result {
        Ok(())
    }

    #[inline]
    fn seq_elem_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn seq_elem<'e: 'a, E: receiver::Source<'e>>(&mut self, e: E) -> receiver::Result {
        self.push(value(e)?)
    }
}
