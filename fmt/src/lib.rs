use std::{
    error,
    fmt::{self, Debug, Display, Formatter, Write},
};

use sval_generic_api::{value, receiver::{self, Receiver}};

pub struct Value<V>(V);

impl<V> Value<V> {
    pub fn new(source: V) -> Self {
        Value(source)
    }
}

pub fn value<V: value::Value>(v: V) -> Value<V> {
    Value::new(v)
}

impl<V: value::Value> Debug for Value<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.stream(FmtReceiver::new(f)).map_err(|_| fmt::Error)
    }
}

struct FmtReceiver<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    depth: usize,
    is_current_depth_empty: bool,
}

impl<'a, 'b: 'a> FmtReceiver<'a, 'b> {
    fn new(fmt: &'a mut Formatter<'b>) -> Self {
        FmtReceiver {
            depth: 0,
            is_current_depth_empty: false,
            fmt,
        }
    }

    fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }

    fn fmt(&mut self, v: impl fmt::Debug) -> receiver::Result {
        v.fmt(&mut self.fmt)?;

        Ok(())
    }
}

impl<'fa, 'fb: 'fa, 'a> Receiver<'a> for FmtReceiver<'fa, 'fb> {
    fn display<D: Display>(&mut self, v: D) -> receiver::Result {
        struct Adapter<T>(T);

        impl<T: Display> Debug for Adapter<T> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        self.fmt(Adapter(v))
    }

    fn error<'e: 'a, E: receiver::ValueSource<'e, dyn error::Error + 'static>>(
        &mut self,
        mut e: E,
    ) -> receiver::Result {
        self.fmt(e.take()?)
    }

    fn i64(&mut self, v: i64) -> receiver::Result {
        self.fmt(v)
    }

    fn u64(&mut self, v: u64) -> receiver::Result {
        self.fmt(v)
    }

    fn i128(&mut self, v: i128) -> receiver::Result {
        self.fmt(v)
    }

    fn u128(&mut self, v: u128) -> receiver::Result {
        self.fmt(v)
    }

    fn f64(&mut self, v: f64) -> receiver::Result {
        self.fmt(v)
    }

    fn bool(&mut self, v: bool) -> receiver::Result {
        self.fmt(v)
    }

    fn str<'s: 'a, S: receiver::ValueSource<'s, str>>(&mut self, mut v: S) -> receiver::Result {
        self.fmt(v.take()?)
    }

    fn none(&mut self) -> receiver::Result {
        self.fmt(format_args!("None"))
    }

    fn type_tagged_begin<T: receiver::ValueSource<'static, str>>(
        &mut self,
        mut tag: receiver::TypeTag<T>,
    ) -> receiver::Result {
        self.fmt.write_str(tag.ty.take()?)?;

        Ok(())
    }

    fn type_tagged_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn variant_tagged_begin<
        T: receiver::ValueSource<'static, str>,
        K: receiver::ValueSource<'static, str>,
    >(
        &mut self,
        mut tag: receiver::VariantTag<T, K>,
    ) -> receiver::Result {
        self.fmt.write_str(tag.ty.take()?)?;
        self.fmt.write_str("::")?;
        self.fmt.write_str(tag.variant_key.take()?)?;

        Ok(())
    }

    fn variant_tagged_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> receiver::Result {
        self.is_current_depth_empty = true;
        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> receiver::Result {
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

    fn map_key_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> receiver::Result {
        self.fmt.write_str(": ")?;

        Ok(())
    }

    fn map_value_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_end(&mut self) -> receiver::Result {
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

    fn seq_begin(&mut self, _: Option<usize>) -> receiver::Result {
        self.is_current_depth_empty = true;

        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('[')?;

        Ok(())
    }

    fn seq_elem_begin(&mut self) -> receiver::Result {
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

    fn seq_elem_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> receiver::Result {
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
}

fn pad(mut w: impl Write, amt: usize) -> fmt::Result {
    for _ in 0..amt {
        w.write_str("    ")?;
    }

    Ok(())
}
