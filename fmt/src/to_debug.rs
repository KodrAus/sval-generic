use core::fmt;

use crate::writer::Writer;

pub fn debug<V: sval::Value>(value: V) -> Debug<V> {
    Debug(value)
}

pub struct Debug<V>(V);

impl<V: sval::Value> fmt::Debug for Debug<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.stream(Writer::new(f))?;

        Ok(())
    }
}
