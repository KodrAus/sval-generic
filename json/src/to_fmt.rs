use core::fmt::{self, Write};

pub fn to_fmt(fmt: impl Write, v: impl sval::Value) -> sval::Result {
    v.stream(&mut Formatter::new(fmt))
}

pub struct Formatter<W> {
    is_internally_tagged: bool,
    is_current_depth_empty: bool,
    is_text_quoted: bool,
    text_handler: Option<TextHandler>,
    out: W,
}

impl<W> Formatter<W> {
    pub fn new(out: W) -> Self {
        Formatter {
            is_internally_tagged: false,
            is_current_depth_empty: true,
            is_text_quoted: true,
            text_handler: None,
            out,
        }
    }

    pub fn into_inner(self) -> W {
        self.out
    }
}

impl<'sval, W> sval::Stream<'sval> for Formatter<W>
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

    fn bool(&mut self, v: bool) -> sval::Result {
        self.out.write_str(if v { "true" } else { "false" })?;

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_text_quoted {
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, v: &str) -> sval::Result {
        if let Some(ref mut handler) = self.text_handler {
            handler.text_fragment(v, &mut self.out)?;
        } else {
            escape_str(v, &mut self.out)?;
        }

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_text_quoted {
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn binary_begin(&mut self, size: Option<usize>) -> sval::Result {
        self.seq_begin(size)
    }

    fn binary_fragment_computed(&mut self, v: &[u8]) -> sval::Result {
        for b in v {
            self.seq_value_begin()?;
            self.u8(*b)?;
            self.seq_value_end()?;
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
        if !self.is_text_quoted {
            return sval::result::unsupported();
        }

        self.is_current_depth_empty = true;
        self.out.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            self.out.write_str(",\"")?;
        } else {
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.out.write_str("\":")?;

        self.is_text_quoted = true;

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
        self.out.write_char('}')?;

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        if !self.is_text_quoted {
            return sval::result::unsupported();
        }

        self.is_current_depth_empty = true;

        self.out.write_char('[')?;

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.out.write_char(']')?;

        Ok(())
    }

    fn enum_begin(&mut self, _: sval::Tag) -> sval::Result {
        self.is_internally_tagged = true;

        Ok(())
    }

    fn enum_end(&mut self, _: sval::Tag) -> sval::Result {
        if self.is_internally_tagged {
            self.map_value_end()?;
            self.map_end()?;
        }

        self.is_internally_tagged = false;

        Ok(())
    }

    fn tagged_begin(&mut self, tag: sval::Tag) -> sval::Result {
        if self.is_internally_tagged {
            if let Some(label) = tag.label() {
                self.map_begin(Some(1))?;

                self.map_key_begin()?;
                escape_str(&*label, &mut self.out)?;
                self.map_key_end()?;

                self.map_value_begin()?;
            }
        }

        self.is_internally_tagged = false;

        Ok(())
    }

    fn tagged_end(&mut self, tag: sval::Tag) -> sval::Result {
        if tag.label().is_some() {
            self.is_internally_tagged = true;
        }

        Ok(())
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            self.out.write_str(",\"")?;
        } else {
            self.out.write_char('"')?;
        }

        escape_str(&*label, &mut self.out)?;

        self.out.write_str("\":")?;

        self.map_value_begin()
    }

    fn constant_begin(&mut self, _: sval::Tag) -> sval::Result {
        Ok(())
    }

    fn constant_end(&mut self, _: sval::Tag) -> sval::Result {
        self.is_internally_tagged = false;

        Ok(())
    }

    fn optional_some_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn optional_some_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn optional_none(&mut self) -> sval::Result {
        self.null()
    }

    fn number_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;
        self.text_handler = Some(TextHandler::number());

        Ok(())
    }

    fn number_end(&mut self) -> sval::Result {
        self.is_text_quoted = true;

        if let Some(TextHandler::Number(mut number)) = self.text_handler.take() {
            number.end(&mut self.out)?;
        }

        Ok(())
    }
}

enum TextHandler {
    Number(NumberTextHandler),
}

struct NumberTextHandler {
    at_start: bool,
    sign_negative: bool,
    leading_zeroes: usize,
    is_nan_or_infinity: bool,
}

impl TextHandler {
    fn number() -> Self {
        TextHandler::Number(NumberTextHandler {
            sign_negative: false,
            leading_zeroes: 0,
            at_start: true,
            is_nan_or_infinity: false,
        })
    }

    fn text_fragment(&mut self, v: &str, out: impl Write) -> sval::Result {
        match self {
            TextHandler::Number(number) => number.text_fragment(v, out),
        }
    }
}

impl NumberTextHandler {
    fn text_fragment(&mut self, v: &str, mut out: impl Write) -> sval::Result {
        if !self.is_nan_or_infinity {
            let mut range = 0..0;

            for b in v.as_bytes() {
                match b {
                    // JSON numbers don't support leading zeroes (except for `0.x`)
                    // so we need to shift over them
                    b'0' if self.at_start => {
                        self.leading_zeroes += 1;
                        range.start += 1;
                        range.end += 1;
                    }
                    // If we're not skipping zeroes then shift over it to write later
                    b'0'..=b'9' => {
                        if self.at_start && self.sign_negative {
                            out.write_char('-')?;
                        }

                        self.at_start = false;
                        range.end += 1;
                    }
                    // If we encounter a decimal point we might need to write a leading `0`
                    b'.' => {
                        if self.at_start {
                            if self.sign_negative {
                                out.write_char('-')?;
                            }

                            out.write_char('0')?;
                        }

                        self.at_start = false;
                        range.end += 1;
                    }
                    // If we encounter a sign then stash it until we know the number is finite
                    // A value like `-inf` should still write `null`, not `-null`
                    b'-' if self.at_start => {
                        self.sign_negative = true;
                        range.start += 1;
                        range.end += 1;
                    }
                    // JSON doesn't support a leading `+` sign
                    b'+' if self.at_start => {
                        range.start += 1;
                        range.end += 1;
                    }
                    // `snan`, `nan`, `inf` in any casing should write `null`
                    b's' | b'n' | b'i' | b'S' | b'N' | b'I' => {
                        self.is_nan_or_infinity = true;
                        self.at_start = false;

                        out.write_str("null")?;

                        range.start = 0;
                        range.end = 0;

                        break;
                    }
                    _ => range.end += 1,
                }
            }

            out.write_str(&v[range])?;
        }

        Ok(())
    }

    fn end(&mut self, mut out: impl Write) -> sval::Result {
        if self.at_start {
            out.write_char('0')?;
        }

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
            BB => out.write_str("\\b")?,
            TT => out.write_str("\\t")?,
            NN => out.write_str("\\n")?,
            FF => out.write_str("\\f")?,
            RR => out.write_str("\\r")?,
            QU => out.write_str("\\\"")?,
            BS => out.write_str("\\\\")?,
            U => {
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
