use sval_generic_api::stream::{self, Stream};

use std::{
    fmt::{self, Write},
};

pub fn to_fmt<'a>(
    fmt: impl Write,
    mut v: impl stream::Source<'a>,
) -> Result<(), sval_generic_api::Error> {
    v.stream(Formatter::new(fmt))
}

pub struct Formatter<W> {
    is_key: bool,
    is_current_depth_empty: bool,
    out: W,
}

impl<W> Formatter<W>
where
    W: Write,
{
    pub fn new(out: W) -> Self {
        Formatter {
            is_key: false,
            is_current_depth_empty: true,
            out,
        }
    }

    pub fn into_inner(self) -> W {
        self.out
    }
}

impl<'a, W> Stream<'a> for Formatter<W>
where
    W: Write,
{
    fn display<V: fmt::Display>(&mut self, v: V) -> stream::Result {
        if self.is_key {
            fmt::write(&mut Escape(&mut self.out), format_args!("{}", v))?;
        } else {
            self.out.write_char('"')?;
            fmt::write(&mut Escape(&mut self.out), format_args!("{}", v))?;
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.out.write_str(ryu::Buffer::new().format(v))?;

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.out.write_str(if v { "true" } else { "false" })?;

        Ok(())
    }

    fn str<'v, V: stream::TypedSource<'v, str>>(&mut self, mut v: V) -> stream::Result
    where
        'v: 'a,
    {
        if self.is_key {
            escape_str(v.stream_to_value()?, &mut self.out)?;
        } else {
            self.out.write_char('"')?;
            escape_str(v.stream_to_value()?, &mut self.out)?;
            self.out.write_char('"')?;
        }

        Ok(())
    }

    fn none(&mut self) -> stream::Result {
        self.out.write_str("null")?;

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.is_key {
            return Err(stream::Error);
        }

        self.is_current_depth_empty = true;
        self.out.write_char('{')?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> stream::Result {
        self.is_key = true;

        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        self.out.write_char('"')?;

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_key_end(&mut self) -> stream::Result {
        self.out.write_char('"')?;

        self.is_key = false;

        Ok(())
    }

    fn map_value_begin(&mut self) -> stream::Result {
        self.out.write_char(':')?;

        Ok(())
    }

    fn map_value_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
        self.is_current_depth_empty = false;

        self.out.write_char('}')?;

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.is_key {
            return Err(stream::Error);
        }

        self.is_current_depth_empty = true;

        self.out.write_char('[')?;

        Ok(())
    }

    fn seq_elem_begin(&mut self) -> stream::Result {
        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_elem_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
        self.is_current_depth_empty = false;

        self.out.write_char(']')?;

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

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
