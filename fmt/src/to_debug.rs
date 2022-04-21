use core::fmt::{self, Write};

pub fn debug<V: sval::Value>(value: V) -> Debug<V> {
    Debug(value)
}

pub struct Debug<V>(V);

impl<V: sval::Value> fmt::Debug for Debug<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.stream(Formatter::new(f))?;

        Ok(())
    }
}

struct Formatter<'a, 'b> {
    is_current_depth_empty: bool,
    is_text_quoted: bool,
    fmt: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> Formatter<'a, 'b> {
    pub fn new(fmt: &'a mut fmt::Formatter<'b>) -> Self {
        Formatter {
            is_current_depth_empty: true,
            is_text_quoted: true,
            fmt,
        }
    }

    fn value(&mut self, v: impl fmt::Debug) -> sval::Result {
        v.fmt(self.fmt)?;

        Ok(())
    }
}

impl<'a, 'b, 'c> sval::Stream<'c> for Formatter<'a, 'b> {
    fn unit(&mut self) -> sval::Result {
        self.value(())
    }

    fn null(&mut self) -> sval::Result {
        self.fmt.write_str("None")?;

        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.value(value)
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_text_quoted {
            self.fmt.write_char('"')?;
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
            use fmt::Debug;

            Escape(fragment).fmt(self.fmt)?;
        } else {
            self.fmt.write_str(fragment)?;
        }

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_text_quoted {
            self.fmt.write_char('"')?;
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
        self.value(value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.value(value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.value(value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.value(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.value(value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.value(value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.value(value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.value(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.value(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.value(value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.value(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.value(value)
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        self.fmt.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.fmt.write_str(", ")?;
        } else {
            self.fmt.write_char(' ')?;
        }

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.fmt.write_str(": ")?;

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
            self.fmt.write_str(" }")?;
        } else {
            self.fmt.write_char('}')?;
        }

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        self.fmt.write_char('[')?;

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.fmt.write_str(", ")?;
        }

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.fmt.write_char(']')?;

        Ok(())
    }

    fn tagged_begin(&mut self, tag: Option<sval::Tag>) -> sval::Result {
        self.is_text_quoted = true;

        if let Some(sval::Tag::Labeled { label, .. }) = tag {
            self.fmt.write_str(label)?;
        }

        self.fmt.write_char('(')?;

        Ok(())
    }

    fn tagged_end(&mut self) -> sval::Result {
        self.fmt.write_char(')')?;

        Ok(())
    }

    fn constant_begin(&mut self, _: Option<sval::Tag>) -> sval::Result {
        self.is_text_quoted = false;

        Ok(())
    }

    fn constant_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        Ok(())
    }

    fn enum_begin(&mut self, _: Option<sval::Tag>) -> sval::Result {
        Ok(())
    }

    fn enum_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn struct_map_begin(
        &mut self,
        tag: Option<sval::Tag>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        if let Some(sval::Tag::Labeled { label, .. }) = tag {
            self.fmt.write_str(label)?;
            self.fmt.write_char(' ')?;
        }

        self.map_begin(num_entries_hint)
    }

    fn struct_map_key_begin(&mut self, _: sval::Tag) -> sval::Result {
        self.is_text_quoted = false;

        self.map_key_begin()?;

        Ok(())
    }

    fn struct_map_key_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        self.map_key_end()
    }

    fn struct_map_value_begin(&mut self, _: sval::Tag) -> sval::Result {
        self.map_value_begin()
    }

    fn struct_map_value_end(&mut self) -> sval::Result {
        self.map_value_end()
    }

    fn struct_map_end(&mut self) -> sval::Result {
        self.map_end()
    }

    fn struct_seq_begin(&mut self, tag: Option<sval::Tag>, _: Option<usize>) -> sval::Result {
        self.is_text_quoted = true;
        self.is_current_depth_empty = true;

        if let Some(sval::Tag::Labeled { label, .. }) = tag {
            self.fmt.write_str(label)?;
        }

        self.fmt.write_char('(')?;

        Ok(())
    }

    fn struct_seq_value_begin(&mut self, _: sval::Tag) -> sval::Result {
        self.seq_value_begin()
    }

    fn struct_seq_value_end(&mut self) -> sval::Result {
        self.seq_value_end()
    }

    fn struct_seq_end(&mut self) -> sval::Result {
        self.fmt.write_char(')')?;

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
