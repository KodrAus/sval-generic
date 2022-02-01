use std::fmt::{self, Write};

pub struct Value<V>(V);

impl<V> Value<V> {
    pub fn new(source: V) -> Self {
        Value(source)
    }

    pub fn get(&self) -> &V {
        &self.0
    }
}

pub fn value<V: sval::Value>(v: V) -> Value<V> {
    Value::new(v)
}

impl<V: sval::Value> fmt::Debug for Value<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.stream(FmtReceiver::new(f)).map_err(|_| fmt::Error)
    }
}

impl<V: fmt::Display> sval::Value for Value<V> {
    fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
        receiver.unstructured(&self.0)
    }
}

struct FmtReceiver<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    depth: usize,
    is_current_depth_empty: bool,
}

impl<'a, 'b: 'a> FmtReceiver<'a, 'b> {
    fn new(fmt: &'a mut fmt::Formatter<'b>) -> Self {
        FmtReceiver {
            depth: 0,
            is_current_depth_empty: false,
            fmt,
        }
    }

    fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }

    fn fmt(&mut self, v: impl fmt::Debug) -> sval::Result {
        v.fmt(&mut self.fmt)?;

        Ok(())
    }
}

impl<'fa, 'fb: 'fa, 'a> sval::Receiver<'a> for FmtReceiver<'fa, 'fb> {
    fn unstructured<D: fmt::Display>(&mut self, v: D) -> sval::Result {
        struct Adapter<T>(T);

        impl<T: fmt::Display> fmt::Debug for Adapter<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        self.fmt(Adapter(v))
    }

    fn u64(&mut self, v: u64) -> sval::Result {
        self.fmt(v)
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        self.fmt(v)
    }

    fn u128(&mut self, v: u128) -> sval::Result {
        self.fmt(v)
    }

    fn i128(&mut self, v: i128) -> sval::Result {
        self.fmt(v)
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        self.fmt(v)
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        self.fmt(v)
    }

    fn null(&mut self) -> sval::Result {
        self.fmt(format_args!("None"))
    }

    fn str<'s: 'a, S: sval::ValueSource<'s, str>>(&mut self, mut v: S) -> sval::Result {
        self.fmt(v.take()?)
    }

    fn error<'e: 'a, E: sval::ValueSource<'e, sval::data::Error>>(
        &mut self,
        mut e: E,
    ) -> sval::Result {
        self.fmt(e.take()?)
    }

    fn tagged_begin<T: sval::ValueSource<'static, str>>(
        &mut self,
        mut tag: sval::data::Tag<T>,
    ) -> sval::Result {
        if let Some(label) = tag.label_mut() {
            self.fmt.write_str(label.take()?)?;

            if tag.kind() == sval::data::tag::Kind::Enum {
                self.fmt.write_str("::")?;
            }
        }

        Ok(())
    }

    fn tagged_end<T: sval::ValueSource<'static, str>>(
        &mut self,
        _: sval::data::Tag<T>,
    ) -> sval::Result {
        Ok(())
    }

    fn map_begin(&mut self, _: Option<u64>) -> sval::Result {
        self.is_current_depth_empty = true;
        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('{')?;

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        if self.is_pretty() {
            self.depth -= 1;

            if !self.is_current_depth_empty {
                self.fmt.write_str(",\n")?;
                pad(&mut self.fmt, self.depth)?;
            }
        }

        self.fmt.write_char('}')?;

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        if self.is_pretty() {
            if !self.is_current_depth_empty {
                self.fmt.write_char(',')?;
            }

            self.fmt.write_char('\n')?;
            pad(&mut self.fmt, self.depth)?;
        } else if !self.is_current_depth_empty {
            self.fmt.write_str(", ")?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.fmt.write_str(": ")?;

        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<u64>) -> sval::Result {
        self.is_current_depth_empty = true;

        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('[')?;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        if self.is_pretty() {
            self.depth -= 1;

            if !self.is_current_depth_empty {
                self.fmt.write_str(",\n")?;
                pad(&mut self.fmt, self.depth)?;
            }
        }

        self.fmt.write_char(']')?;

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_elem_begin(&mut self) -> sval::Result {
        if self.is_pretty() {
            if !self.is_current_depth_empty {
                self.fmt.write_char(',')?;
            }

            self.fmt.write_char('\n')?;
            pad(&mut self.fmt, self.depth)?;
        } else if !self.is_current_depth_empty {
            self.fmt.write_str(", ")?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_elem_end(&mut self) -> sval::Result {
        Ok(())
    }
}

fn pad(mut w: impl Write, amt: usize) -> fmt::Result {
    for _ in 0..amt {
        w.write_str("    ")?;
    }

    Ok(())
}
