use core::fmt::{self, Write};

pub fn to_fmt<'a>(fmt: impl Write, mut v: impl sval::Source<'a>) -> sval::Result {
    v.stream_to_end(Formatter::new(fmt))
}

pub struct Formatter<W> {
    is_key: bool,
    is_internally_tagged: bool,
    is_current_depth_empty: bool,
    write_str_quotes: bool,
    text_handler: Option<fn(&str, &mut u32, &mut dyn Write) -> sval::Result>,
    text_handler_state: u32,
    out: W,
}

impl<W> Formatter<W>
where
    W: Write,
{
    pub fn new(out: W) -> Self {
        Formatter {
            is_key: false,
            is_internally_tagged: false,
            is_current_depth_empty: true,
            write_str_quotes: true,
            text_handler: None,
            text_handler_state: 0,
            out,
        }
    }

    pub fn into_inner(self) -> W {
        self.out
    }

    fn escape_str(&mut self, v: &str) -> sval::Result {
        escape_str(v, &mut self.out)?;

        Ok(())
    }
}

impl<'a, W> sval::Receiver<'a> for Formatter<W>
where
    W: Write,
{
    fn dynamic_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn dynamic_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn unit(&mut self) -> sval::Result {
        self.null()
    }

    fn null(&mut self) -> sval::Result {
        self.out.write_str("null")?;

        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        self.out.write_str(if v { "true" } else { "false" })?;

        Ok(())
    }

    fn text(&mut self, v: &'a str) -> sval::Result {
        if self.write_str_quotes {
            self.out.write_char('"')?;
            self.escape_str(v)?;
            self.out.write_char('"')?;
        } else {
            self.escape_str(v)?;
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
        if let Some(text_handler) = self.text_handler {
            text_handler(v, &mut self.text_handler_state, &mut self.out)?;
        } else {
            self.escape_str(v)?;
        }

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
            self.seq_value(b)?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.seq_end()
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
        if v.is_nan() || v.is_infinite() {
            self.null()?;
        } else {
            self.out.write_str(ryu::Buffer::new().format_finite(v))?;
        }

        Ok(())
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        if v.is_nan() || v.is_infinite() {
            self.null()?;
        } else {
            self.out.write_str(ryu::Buffer::new().format_finite(v))?;
        }

        Ok(())
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

    fn map_key<'k: 'a, K: sval::Source<'k>>(&mut self, mut key: K) -> sval::Result {
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

        Ok(())
    }

    fn map_value<'v: 'a, V: sval::Source<'v>>(&mut self, mut value: V) -> sval::Result {
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

    fn seq_value_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        self.out.write_char(']')?;

        Ok(())
    }

    fn tagged_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        if self.is_internally_tagged {
            self.map_begin(Some(1))?;

            match tag {
                sval::data::Tag {
                    label: Some(label), ..
                } => self.map_key(label)?,
                sval::data::Tag { id: Some(id), .. } => self.map_key(id)?,
                _ => sval::error::unsupported()?,
            }

            self.map_value_begin()?;
        }

        Ok(())
    }

    fn tagged_end(&mut self) -> sval::Result {
        self.is_internally_tagged = true;

        Ok(())
    }

    fn constant_begin(&mut self, _: sval::data::Tag) -> sval::Result {
        self.is_internally_tagged = false;

        Ok(())
    }

    fn constant_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn enum_begin(&mut self, _: sval::data::Tag) -> sval::Result {
        self.is_internally_tagged = true;

        Ok(())
    }

    fn enum_end(&mut self) -> sval::Result {
        if self.is_internally_tagged {
            self.map_value_end()?;
            self.map_end()?;
        }

        self.is_internally_tagged = false;

        Ok(())
    }

    fn int_begin(&mut self) -> sval::Result {
        self.write_str_quotes = false;

        Ok(())
    }

    fn int_end(&mut self) -> sval::Result {
        self.write_str_quotes = true;

        Ok(())
    }

    fn binfloat_begin(&mut self) -> sval::Result {
        self.write_str_quotes = false;
        self.text_handler = Some(float_text_handler);
        self.text_handler_state = Default::default();

        Ok(())
    }

    fn binfloat_end(&mut self) -> sval::Result {
        self.write_str_quotes = false;
        self.text_handler = None;
        self.text_handler_state = Default::default();

        Ok(())
    }

    fn decfloat_begin(&mut self) -> sval::Result {
        self.write_str_quotes = false;
        self.text_handler = Some(float_text_handler);
        self.text_handler_state = Default::default();

        Ok(())
    }

    fn decfloat_end(&mut self) -> sval::Result {
        self.write_str_quotes = false;
        self.text_handler = None;
        self.text_handler_state = Default::default();

        Ok(())
    }
}

fn float_text_handler(mut v: &str, state: &mut u32, out: &mut dyn Write) -> sval::Result {
    // The state tracks what kind of number we've detected across calls
    struct State<'a>(&'a mut u32);

    impl<'a> State<'a> {
        fn check_write_negative_sign(&self) -> bool {
            *self.0 >> 16 == 1
        }

        fn must_write_negative_sign(&mut self) {
            *self.0 |= 1u32 << 16;
        }

        fn complete_write_negative_sign(&mut self) {
            *self.0 &= u16::MAX as u32;
        }

        fn check_nan_or_inf(&self) -> bool {
            *self.0 as u16 == 0
        }

        fn check_write_digits(&self) -> bool {
            *self.0 as u16 == 2
        }

        fn must_not_write_digits(&mut self) {
            *self.0 = 1;
        }

        fn must_write_digits(&mut self) {
            *self.0 = *self.0 | 2;
        }
    }

    let mut skip = 0;
    let mut s = State(state);

    // Check whether the number is NaN or +/- infinity
    // In these cases we want to write `null` instead
    if s.check_nan_or_inf() {
        for b in v.as_bytes() {
            match b {
                b'+' => {
                    skip = 1;
                }
                b'-' => {
                    skip = 1;
                    s.must_write_negative_sign();
                }
                b'n' | b'i' | b'N' | b'I' => {
                    out.write_str("null")?;
                    s.must_not_write_digits();

                    return Ok(());
                }
                _ => {
                    s.must_write_digits();
                    break;
                }
            }
        }
    }

    // A leading sign will be stripped
    // This means we make 2 calls to the writer for negative numbers
    v = &v[skip..];

    if s.check_write_digits() {
        if s.check_write_negative_sign() {
            out.write_str("-")?;
            s.complete_write_negative_sign();
        }

        out.write_str(v)?;
    }

    Ok(())
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
