use std::{
    error,
    fmt::{self, Debug, Display, Formatter, Write},
};

use crate::{
    stream::{self, Stream},
    value,
};

pub struct Value<V>(V);

impl<V> Value<V> {
    pub fn new(value: V) -> Self {
        Value(value)
    }
}

impl<V: value::Value> Debug for Value<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.stream(FmtStream::new(f)).map_err(|_| fmt::Error)
    }
}

struct FmtStream<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    depth: usize,
    is_current_depth_empty: bool,
}

impl<'a, 'b: 'a> FmtStream<'a, 'b> {
    fn new(fmt: &'a mut Formatter<'b>) -> Self {
        FmtStream {
            depth: 0,
            is_current_depth_empty: false,
            fmt,
        }
    }

    fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }

    fn fmt(&mut self, v: impl fmt::Debug) -> stream::Result {
        v.fmt(&mut self.fmt)?;

        Ok(())
    }
}

impl<'fa, 'fb: 'fa, 'a> Stream<'a> for FmtStream<'fa, 'fb> {
    fn display<D: Display>(&mut self, v: D) -> stream::Result {
        struct Adapter<T>(T);

        impl<T: Display> Debug for Adapter<T> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        self.fmt(Adapter(v))
    }

    fn error<'e: 'a, E: stream::TypedRef<'e, dyn error::Error + 'static>>(
        &mut self,
        e: E,
    ) -> stream::Result {
        self.fmt(e.get())
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.fmt(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.fmt(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.fmt(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.fmt(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.fmt(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.fmt(v)
    }

    fn str<'s: 'a, S: stream::TypedRef<'s, str>>(&mut self, v: S) -> stream::Result {
        self.fmt(v.get())
    }

    fn none(&mut self) -> stream::Result {
        self.fmt(format_args!("None"))
    }

    fn type_tagged_begin<T: stream::TypedRef<'static, str>>(
        &mut self,
        tag: stream::TypeTag<T>,
    ) -> stream::Result {
        self.fmt.write_str(tag.ty().get())?;

        Ok(())
    }

    fn type_tagged_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn variant_tagged_begin<
        T: stream::TypedRef<'static, str>,
        K: stream::TypedRef<'static, str>,
    >(
        &mut self,
        tag: stream::VariantTag<T, K>,
    ) -> stream::Result {
        self.fmt.write_str(tag.ty().get())?;
        self.fmt.write_str("::")?;
        self.fmt.write_str(tag.variant_key().get())?;

        Ok(())
    }

    fn variant_tagged_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        self.is_current_depth_empty = true;
        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> stream::Result {
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

    fn map_key_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> stream::Result {
        self.fmt.write_str(": ")?;

        Ok(())
    }

    fn map_value_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
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

    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        self.is_current_depth_empty = true;

        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('[')?;

        Ok(())
    }

    fn seq_elem_begin(&mut self) -> stream::Result {
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

    fn seq_elem_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
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
