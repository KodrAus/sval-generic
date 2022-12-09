use core::fmt::{self, Debug, Write};

pub(crate) struct Writer<W> {
    is_current_depth_empty: bool,
    is_text_quoted: bool,
    out: W,
}

pub(crate) trait Fmt: Write {
    fn write_u8(&mut self, value: u8) -> fmt::Result;
    fn write_u16(&mut self, value: u16) -> fmt::Result;
    fn write_u32(&mut self, value: u32) -> fmt::Result;
    fn write_u64(&mut self, value: u64) -> fmt::Result;
    fn write_u128(&mut self, value: u128) -> fmt::Result;
    fn write_i8(&mut self, value: i8) -> fmt::Result;
    fn write_i16(&mut self, value: i16) -> fmt::Result;
    fn write_i32(&mut self, value: i32) -> fmt::Result;
    fn write_i64(&mut self, value: i64) -> fmt::Result;
    fn write_i128(&mut self, value: i128) -> fmt::Result;
    fn write_f32(&mut self, value: f32) -> fmt::Result;
    fn write_f64(&mut self, value: f64) -> fmt::Result;
}

impl<'a, 'b> Fmt for &'a mut fmt::Formatter<'b> {
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        value.fmt(self)
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        value.fmt(self)
    }
}

pub(crate) struct GenericWriter<W>(pub W);

impl<W: Write> Write for GenericWriter<W> {
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

impl<W: Write> Fmt for GenericWriter<W> {
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        self.write_str(ryu::Buffer::new().format(value))
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        self.write_str(ryu::Buffer::new().format(value))
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
        self.out.write_str("()")?;

        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        self.write_str("None")?;

        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.out.write_str(if value { "true" } else { "false" })?;

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_text_quoted {
            self.write_char('"')?;
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        if self.is_text_quoted {
            // Inlined from `impl Debug for str`
            // This avoids writing the outer quotes for the string
            // and handles the `'` case
            // NOTE: The vast (vast) majority of formatting time is spent here
            // Optimizing this would be a big win
            let mut from = 0;

            for (i, c) in fragment.char_indices() {
                let esc = c.escape_debug();

                // If char needs escaping, flush backlog so far and write, else skip
                if c != '\'' && esc.len() != 1 {
                    self.out.write_str(&fragment[from..i])?;
                    for c in esc {
                        self.out.write_char(c)?;
                    }
                    from = i + c.len_utf8();
                }
            }

            self.out.write_str(&fragment[from..])?;
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
        self.out.write_u8(value)?;

        Ok(())
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.out.write_u16(value)?;

        Ok(())
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.out.write_u32(value)?;

        Ok(())
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.out.write_u64(value)?;

        Ok(())
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.out.write_u128(value)?;

        Ok(())
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.out.write_i8(value)?;

        Ok(())
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.out.write_i16(value)?;

        Ok(())
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.out.write_i32(value)?;

        Ok(())
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.out.write_i64(value)?;

        Ok(())
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.out.write_i128(value)?;

        Ok(())
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.out.write_f32(value)?;

        Ok(())
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.out.write_f64(value)?;

        Ok(())
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

    fn enum_begin(&mut self, _: sval::Tag) -> sval::Result {
        Ok(())
    }

    fn enum_end(&mut self, _: sval::Tag) -> sval::Result {
        Ok(())
    }

    fn tagged_begin(
        &mut self,
        tag: sval::Tag,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        self.is_text_quoted = true;

        if let Some(label) = label {
            self.write_str(&*label)?;
        }

        self.write_char('(')?;

        Ok(())
    }

    fn tagged_end(&mut self, _: sval::Tag) -> sval::Result {
        self.write_char(')')?;

        Ok(())
    }

    fn record_begin(
        &mut self,
        tag: sval::Tag,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        if let Some(label) = label {
            self.write_str(&*label)?;
            self.write_char(' ')?;
        }

        self.map_begin(num_entries_hint)
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        self.is_text_quoted = false;
        self.map_key_begin()?;
        sval::stream(&mut *self, &*label)?;
        self.map_key_end()?;
        self.is_text_quoted = true;

        self.map_value_begin()
    }

    fn record_value_end(&mut self, _: sval::Label) -> sval::Result {
        self.map_value_end()
    }

    fn record_end(&mut self, _: sval::Tag) -> sval::Result {
        self.map_end()
    }

    fn tuple_begin(
        &mut self,
        tag: sval::Tag,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        _: Option<usize>,
    ) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        if let Some(label) = tag.label() {
            self.write_str(&*label)?;
        }

        self.write_char('(')?;

        Ok(())
    }

    fn tuple_value_begin(&mut self, _: u32) -> sval::Result {
        self.seq_value_begin()
    }

    fn tuple_value_end(&mut self, _: u32) -> sval::Result {
        self.seq_value_end()
    }

    fn tuple_end(&mut self, _: sval::Tag) -> sval::Result {
        self.write_char(')')?;

        Ok(())
    }

    fn constant_begin(&mut self, _: sval::Tag) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn constant_end(&mut self, _: sval::Tag) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }

    fn number_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn number_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }
}
