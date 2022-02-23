use core::fmt::{self, Write};

pub fn to_fmt<'a>(fmt: impl Write, mut v: impl sval::Source<'a>) -> sval::Result {
    v.stream_to_end(Formatter::new(fmt))
}

pub struct Formatter<W> {
    is_key: bool,
    is_current_depth_empty: bool,
    write_str_quotes: bool,
    last_tag: Option<sval::data::Tag>,
    out: W,
}

impl<W> Formatter<W>
where
    W: Write,
{
    pub fn new(out: W) -> Self {
        Formatter {
            is_key: false,
            write_str_quotes: true,
            is_current_depth_empty: true,
            last_tag: None,
            out,
        }
    }

    pub fn into_inner(self) -> W {
        self.out
    }
}

impl<'a, W> sval::Receiver<'a> for Formatter<W>
where
    W: Write,
{
    fn unit(&mut self) -> sval::Result {
        self.null()
    }

    fn null(&mut self) -> sval::Result {
        self.out.write_str("null")?;

        Ok(())
    }

    fn u8(&mut self, v: u8) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u16(&mut self, v: u16) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u32(&mut self, v: u32) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u64(&mut self, v: u64) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u128(&mut self, v: u128) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i8(&mut self, v: i8) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i16(&mut self, v: i16) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i32(&mut self, v: i32) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i128(&mut self, v: i128) -> sval::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn f32(&mut self, v: f32) -> sval::Result {
        self.out.write_str(ryu::Buffer::new().format(v))?;

        Ok(())
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        self.out.write_str(ryu::Buffer::new().format(v))?;

        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        self.out.write_str(if v { "true" } else { "false" })?;

        Ok(())
    }

    fn str(&mut self, v: &'a str) -> sval::Result {
        if self.write_str_quotes {
            self.out.write_char('"')?;
            escape_str(v, &mut self.out)?;
            self.out.write_char('"')?;
        } else {
            escape_str(v, &mut self.out)?;
        }

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.write_str_quotes {
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, v: &str) -> sval::Result {
        escape_str(v, &mut self.out)?;

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if self.write_str_quotes {
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn binary_begin(&mut self, size: Option<usize>) -> sval::Result {
        self.seq_begin(size)
    }

    fn binary_fragment_computed(&mut self, v: &[u8]) -> sval::Result {
        for b in v {
            self.seq_elem(b)?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn tagged_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.last_tag = Some(tag);

        match tag.shape {
            // Big integers: We can write these directly into the output without
            // quoting so we'll omit any quotes on the subsequent number
            sval::data::TagShape::BigInteger | sval::data::TagShape::Number => {
                self.write_str_quotes = false;

                Ok(())
            }
            sval::data::TagShape::Enum => todo!(),
            _ => Ok(()),
        }
    }

    fn tagged_end(&mut self, tag: sval::data::Tag) -> sval::Result {
        match tag.shape {
            // Big integers: restore string quoting
            sval::data::TagShape::BigInteger | sval::data::TagShape::Number => {
                self.write_str_quotes = true;
            }
            sval::data::TagShape::Enum => todo!(),
            _ => (),
        }

        self.last_tag = Some(tag);

        Ok(())
    }

    fn tagged<'v: 'a, V: sval::Source<'v>>(
        &mut self,
        mut tagged: sval::data::Tagged<V>,
    ) -> sval::Result {
        let tag = tagged.tag();

        match tag.shape {
            // If we encounter a struct field then attempt to write its label
            // If it doesn't have a label then we'll fall back to its content
            sval::data::TagShape::StructField => {
                if let Some(key) = tag.label {
                    escape_str(key, &mut self.out)?;

                    return Ok(());
                }
            }
            _ => (),
        }

        self.tagged_begin(tag)?;
        tagged.value_mut().stream_to_end(&mut *self)?;
        self.tagged_end(tag)
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_key {
            return Err(sval::Error);
        }

        self.is_current_depth_empty = true;
        self.out.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.is_key = true;
        self.write_str_quotes = false;

        if !self.is_current_depth_empty {
            self.out.write_str(",\"")?;
        } else {
            self.out.write_char('"')?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.out.write_str("\":")?;

        self.is_key = false;
        self.write_str_quotes = true;

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        self.out.write_char('}')?;

        Ok(())
    }

    fn map_entry<'k: 'a, 'v: 'a, K: sval::Source<'k>, V: sval::Source<'v>>(
        &mut self,
        mut key: K,
        mut value: V,
    ) -> sval::Result {
        if !self.is_current_depth_empty {
            self.out.write_str(",\"")?;
        } else {
            self.out.write_char('"')?;
        }

        self.is_key = true;
        self.write_str_quotes = false;

        key.stream_to_end(&mut *self)?;

        self.is_key = false;
        self.write_str_quotes = true;

        self.out.write_str("\":")?;

        self.is_current_depth_empty = false;

        value.stream_to_end(&mut *self)
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_key {
            return Err(sval::Error);
        }

        self.is_current_depth_empty = true;

        self.out.write_char('[')?;

        Ok(())
    }

    fn seq_elem_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_elem_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        self.out.write_char(']')?;

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

#[inline(always)]
fn escape_str(value: &str, mut out: impl Write) -> Result<(), fmt::Error> {
    let bytes = value.as_bytes();
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            out.write_str(&value[start..i])?;
        }

        match escape {
            self::BB => out.write_str("\\b")?,
            self::TT => out.write_str("\\t")?,
            self::NN => out.write_str("\\n")?,
            self::FF => out.write_str("\\f")?,
            self::RR => out.write_str("\\r")?,
            self::QU => out.write_str("\\\"")?,
            self::BS => out.write_str("\\\\")?,
            self::U => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                out.write_str("\\u00")?;
                out.write_char(HEX_DIGITS[(byte >> 4) as usize] as char)?;
                out.write_char(HEX_DIGITS[(byte & 0xF) as usize] as char)?;
            }
            _ => unreachable!(),
        }

        start = i + 1;
    }

    if start != bytes.len() {
        out.write_str(&value[start..])?;
    }

    Ok(())
}

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const U: u8 = b'u'; // \x00...\x1F except the ones above

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
#[rustfmt::skip]
static ESCAPE: [u8; 256] = [
    //  1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    U,  U,  U,  U,  U,  U,  U,  U, BB, TT, NN,  U, FF, RR,  U,  U, // 0
    U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U, // 1
    0,  0, QU,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 2
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 3
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 4
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, BS,  0,  0,  0, // 5
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 6
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 7
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 8
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 9
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // A
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // B
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // C
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // D
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // E
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // F
];

struct Escape<W>(W);

impl<W> Write for Escape<W>
where
    W: Write,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        escape_str(s, &mut self.0)
    }
}
