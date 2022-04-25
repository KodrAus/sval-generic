use core::fmt::{self, Write};

pub(crate) struct Writer<W> {
    is_current_depth_empty: bool,
    is_text_quoted: bool,
    out: W,
}

pub(crate) trait Fmt: fmt::Write {
    fn write_debug<D: fmt::Debug>(&mut self, value: D) -> fmt::Result;
}

impl<'a, 'b> Fmt for &'a mut fmt::Formatter<'b> {
    fn write_debug<D: fmt::Debug>(&mut self, value: D) -> fmt::Result {
        value.fmt(self)
    }
}

pub(crate) struct WriteDebugAsFormatArgs<W>(pub W);

impl<W: fmt::Write> Write for WriteDebugAsFormatArgs<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args)
    }
}

impl<W: fmt::Write> Fmt for WriteDebugAsFormatArgs<W> {
    fn write_debug<D: fmt::Debug>(&mut self, value: D) -> fmt::Result {
        self.0.write_fmt(format_args!("{:?}", value))
    }
}

impl<W> Writer<W> {
    pub fn new(out: W) -> Self {
        Writer {
            is_current_depth_empty: true,
            is_text_quoted: true,
            out,
        }
    }

    pub fn into_inner(self) -> W {
        self.out
    }
}

impl<W: Fmt> Writer<W> {
    fn write_value(&mut self, v: impl fmt::Debug) -> sval::Result {
        self.out.write_debug(v)?;

        Ok(())
    }
}

impl<W: fmt::Write> Write for Writer<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.out.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.out.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.out.write_fmt(args)
    }
}

impl<'sval, W: Fmt> sval::Stream<'sval> for Writer<W> {
    fn unit(&mut self) -> sval::Result {
        self.write_value(())
    }

    fn null(&mut self) -> sval::Result {
        self.write_str("None")?;

        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.write_value(value)
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_text_quoted {
            self.write_char('"')?;
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        struct Escape<'a>(&'a str);

        impl<'a> fmt::Debug for Escape<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Inlined from `impl Debug for str`
                // This avoids writing the outer quotes for the string
                // and handles the `'` case
                let mut from = 0;

                for (i, c) in self.0.char_indices() {
                    let esc = c.escape_debug();

                    // If char needs escaping, flush backlog so far and write, else skip
                    if c != '\'' && esc.len() != 1 {
                        f.write_str(&self.0[from..i])?;
                        for c in esc {
                            f.write_char(c)?;
                        }
                        from = i + c.len_utf8();
                    }
                }

                f.write_str(&self.0[from..])
            }
        }

        if self.is_text_quoted {
            self.write_value(Escape(fragment))?;
        } else {
            self.write_str(fragment)?;
        }

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_text_quoted {
            self.write_char('"')?;
        }

        Ok(())
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.seq_begin(num_bytes_hint)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        for b in fragment {
            self.seq_value_begin()?;
            self.u8(*b)?;
            self.seq_value_end()?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.write_value(value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.write_value(value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.write_value(value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.write_value(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.write_value(value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.write_value(value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.write_value(value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.write_value(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.write_value(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.write_value(value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.write_value(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.write_value(value)
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        self.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.write_str(", ")?;
        } else {
            self.write_char(' ')?;
        }

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.write_str(": ")?;

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.write_str(" }")?;
        } else {
            self.write_char('}')?;
        }

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        self.write_char('[')?;

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.write_str(", ")?;
        }

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.write_char(']')?;

        Ok(())
    }

    fn tagged_begin(&mut self, tag: Option<sval::Tag>) -> sval::Result {
        self.is_text_quoted = true;

        if let Some(sval::Tag::Named { name: label, .. }) = tag {
            self.write_str(label)?;
        }

        self.write_char('(')?;

        Ok(())
    }

    fn tagged_end(&mut self, _: Option<sval::Tag>) -> sval::Result {
        self.write_char(')')?;

        Ok(())
    }

    fn constant_begin(&mut self, _: Option<sval::Tag>) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn constant_end(&mut self, _: Option<sval::Tag>) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }

    fn enum_begin(&mut self, _: Option<sval::Tag>) -> sval::Result {
        Ok(())
    }

    fn enum_end(&mut self, _: Option<sval::Tag>) -> sval::Result {
        Ok(())
    }

    fn record_begin(
        &mut self,
        tag: Option<sval::Tag>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        if let Some(sval::Tag::Named { name: label, .. }) = tag {
            self.write_str(label)?;
            self.write_char(' ')?;
        }

        self.map_begin(num_entries_hint)
    }

    fn record_value_begin(&mut self, tag: sval::TagNamed) -> sval::Result {
        self.is_text_quoted = false;
        self.map_key_begin()?;
        self.value(tag.name)?;
        self.map_key_end()?;
        self.is_text_quoted = true;

        self.map_value_begin()
    }

    fn record_value_end(&mut self, _: sval::TagNamed) -> sval::Result {
        self.map_value_end()
    }

    fn record_end(&mut self, _: Option<sval::Tag>) -> sval::Result {
        self.map_end()
    }

    fn tuple_begin(&mut self, tag: Option<sval::Tag>, _: Option<usize>) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        if let Some(sval::Tag::Named { name: label, .. }) = tag {
            self.write_str(label)?;
        }

        self.write_char('(')?;

        Ok(())
    }

    fn tuple_value_begin(&mut self, _: sval::TagUnnamed) -> sval::Result {
        self.seq_value_begin()
    }

    fn tuple_value_end(&mut self, _: sval::TagUnnamed) -> sval::Result {
        self.seq_value_end()
    }

    fn tuple_end(&mut self, _: Option<sval::Tag>) -> sval::Result {
        self.write_char(')')?;

        Ok(())
    }

    fn int_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn int_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }

    fn binfloat_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn binfloat_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }

    fn decfloat_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn decfloat_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }
}
