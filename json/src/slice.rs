use core::{cmp::Ordering, mem};

pub fn slice(json: &str) -> &JsonSlice {
    JsonSlice::new(json)
}

#[repr(transparent)]
pub struct JsonSlice(str);

impl JsonSlice {
    pub fn new(src: &str) -> &JsonSlice {
        unsafe { mem::transmute::<&str, &JsonSlice>(src) }
    }
}

impl sval::Value for JsonSlice {
    fn stream<'a, R: sval::Receiver<'a>>(&'a self, receiver: R) -> sval::Result {
        use sval::Source;

        JsonSliceReader::new(&self.0).stream_to_end(receiver)
    }

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }
}

pub struct JsonSliceReader<'a> {
    src: &'a [u8],
    head: usize,
    in_str: bool,
    stack: Stack,
    position: Position,
}

impl<'a> JsonSliceReader<'a> {
    pub fn new(src: &'a str) -> JsonSliceReader<'a> {
        JsonSliceReader {
            src: src.as_bytes(),
            head: 0,
            in_str: false,
            stack: Stack::new(),
            position: Position::Root,
        }
    }

    fn map_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => receiver.seq_value_begin()?,
            Position::MapValue => receiver.map_value_begin()?,
            Position::Root => (),
            _ => todo!(),
        }

        self.stack.push_map()?;
        self.position = Position::MapEmpty;

        receiver.map_begin(None)
    }

    fn map_key_end<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        if !matches!(self.position, Position::MapKey) {
            todo!();
        }

        self.position = Position::MapValue;

        receiver.map_key_end()
    }

    fn map_end<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.position {
            Position::MapEmpty => (),
            Position::MapValue => {
                receiver.map_value_end()?;
            }
            _ => todo!(),
        }

        self.stack.pop_map()?;
        self.position = self.stack.position();

        receiver.map_end()
    }

    fn seq_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => receiver.seq_value_begin()?,
            Position::MapValue => receiver.map_value_begin()?,
            Position::Root => (),
            _ => todo!(),
        }

        self.stack.push_seq()?;
        self.position = Position::SeqEmpty;

        receiver.seq_begin(None)
    }

    fn seq_end<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.position {
            Position::SeqEmpty => (),
            Position::SeqElem => {
                receiver.seq_value_end()?;
            }
            _ => todo!(),
        }

        self.stack.pop_seq()?;
        self.position = self.stack.position();

        receiver.seq_end()
    }

    fn map_value_seq_value_end<'b>(
        &mut self,
        mut receiver: impl sval::Receiver<'b>,
    ) -> sval::Result {
        match self.position {
            Position::SeqElem => receiver.seq_value_end(),
            Position::MapValue => {
                self.position = Position::MapKey;

                receiver.map_value_end()
            }
            _ => todo!(),
        }
    }

    fn str_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => {
                self.position = Position::SeqElem;

                receiver.seq_value_begin()
            }
            Position::MapEmpty => {
                self.position = Position::MapKey;

                receiver.map_key_begin()
            }
            Position::MapKey => receiver.map_key_begin(),
            Position::MapValue => receiver.map_value_begin(),
            Position::Root => Ok(()),
        }
    }

    fn value_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => {
                self.position = Position::SeqElem;

                receiver.seq_value_begin()
            }
            Position::MapValue => receiver.map_value_begin(),
            Position::Root => Ok(()),
            _ => todo!(),
        }
    }

    fn maybe_done<'b>(
        &mut self,
        mut receiver: impl sval::Receiver<'b>,
    ) -> sval::Result<sval::Resume> {
        if self.head < self.src.len() {
            Ok(sval::Resume::Continue)
        } else {
            self.stack.finish()?;

            receiver.dynamic_end()?;

            Ok(sval::Resume::Done)
        }
    }
}

impl<'a> sval::Source<'a> for JsonSliceReader<'a> {
    fn stream_resume<'b, R: sval::Receiver<'b>>(
        &mut self,
        mut receiver: R,
    ) -> sval::Result<sval::Resume>
    where
        'a: 'b,
    {
        if self.head == 0 {
            receiver.dynamic_begin()?;
        }

        if self.in_str {
            let (fragment, partial, head) = str_fragment(self.src, self.head)?;

            self.head = head;

            return if !partial {
                receiver.text_fragment(fragment)?;
                receiver.text_end()?;

                self.in_str = false;

                self.maybe_done(receiver)
            } else {
                receiver.text_fragment(fragment)?;

                return Ok(sval::Resume::Continue);
            };
        }

        while self.head < self.src.len() {
            match self.src[self.head] {
                // Begin a string
                b'"' => {
                    self.head += 1;

                    self.str_begin(&mut receiver)?;

                    let (fragment, partial, head) = str_fragment(self.src, self.head)?;

                    self.head = head;

                    // If the string is complete (with no escapes)
                    // then we can yield it directly
                    return if !partial {
                        receiver.str(fragment)?;

                        self.maybe_done(receiver)
                    }
                    // If the string has escapes then yield this fragment
                    // The next time we loop through we'll grab the next one
                    else {
                        self.in_str = true;

                        receiver.text_begin(None)?;
                        receiver.text_fragment(fragment)?;

                        return Ok(sval::Resume::Continue);
                    };
                }
                // Start a map
                b'{' => {
                    self.head += 1;

                    self.map_begin(receiver)?;

                    return Ok(sval::Resume::Continue);
                }
                // End a map
                b'}' => {
                    self.head += 1;

                    self.map_end(&mut receiver)?;

                    return self.maybe_done(receiver);
                }
                // Begin a seq
                b'[' => {
                    self.head += 1;

                    self.seq_begin(receiver)?;

                    return Ok(sval::Resume::Continue);
                }
                // End a seq
                b']' => {
                    self.head += 1;

                    self.seq_end(&mut receiver)?;

                    return self.maybe_done(receiver);
                }
                // End a map key
                b':' => {
                    self.head += 1;

                    self.map_key_end(receiver)?;

                    return Ok(sval::Resume::Continue);
                }
                // End either a map value or seq elem
                b',' => {
                    self.head += 1;

                    self.map_value_seq_value_end(receiver)?;

                    return Ok(sval::Resume::Continue);
                }
                // The boolean value `true`
                b't' => {
                    if let Some(b"true") = self.src.get(self.head..self.head + 4) {
                        self.head += 4;

                        self.value_begin(&mut receiver)?;

                        receiver.bool(true)?;

                        return self.maybe_done(receiver);
                    } else {
                        todo!()
                    }
                }
                // The boolean value `false`
                b'f' => {
                    if let Some(b"false") = self.src.get(self.head..self.head + 5) {
                        self.head += 5;

                        self.value_begin(&mut receiver)?;

                        receiver.bool(false)?;

                        return self.maybe_done(receiver);
                    } else {
                        todo!()
                    }
                }
                // The value `null`
                b'n' => {
                    if let Some(b"null") = self.src.get(self.head..self.head + 4) {
                        self.head += 4;

                        self.value_begin(&mut receiver)?;

                        receiver.null()?;

                        return self.maybe_done(receiver);
                    } else {
                        todo!()
                    }
                }
                // Whitespace
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.head += 1;
                }
                // Numbers
                b'0'..=b'9' | b'-' => {
                    let (n, head) = number(self.src, self.head)?;

                    self.head = head;

                    if !receiver.is_text_based() {
                        // Convert the number to a concrete value
                        todo!()
                    }

                    self.value_begin(&mut receiver)?;

                    receiver.decimal_begin()?;
                    receiver.str(n)?;
                    receiver.decimal_end()?;

                    return self.maybe_done(receiver);
                }
                _ => todo!(),
            }
        }

        self.maybe_done(receiver)
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Root,
    MapEmpty,
    MapKey,
    MapValue,
    SeqEmpty,
    SeqElem,
}

// Instead of keeping a traditional stack or bitmap that identifies the kind
// of value we're inside (either a map or seq), we keep a pair of integers.
// Each integer is dependent on the other. When we add to one, we always
// increment it so it's higher than the other. That means if we start a map
// but then try to end a seq we'll overflow and pick up the mismatch. We can
// guarantee {} [] are balanced this way up to quite a deep level of nesting
// on realistic documents using 128 bits. We can tell whether we're in a map
// or a sequence by looking at the higher number.
#[derive(Debug, Clone, Copy)]
struct Stack {
    map: u64,
    seq: u64,
}

impl Stack {
    fn new() -> Self {
        Stack { map: 0, seq: 0 }
    }

    fn push_map(&mut self) -> sval::Result {
        self.map = self.map.checked_add(1 + self.seq).ok_or(sval::Error)?;
        Ok(())
    }

    fn pop_map(&mut self) -> sval::Result {
        self.map = self.map.checked_sub(1 + self.seq).ok_or(sval::Error)?;
        Ok(())
    }

    fn push_seq(&mut self) -> sval::Result {
        self.seq = self.seq.checked_add(1 + self.map).ok_or(sval::Error)?;
        Ok(())
    }

    fn pop_seq(&mut self) -> sval::Result {
        self.seq = self.seq.checked_sub(1 + self.map).ok_or(sval::Error)?;
        Ok(())
    }

    fn position(&self) -> Position {
        match self.map.cmp(&self.seq) {
            Ordering::Greater => Position::MapValue,
            Ordering::Less => Position::SeqElem,
            Ordering::Equal => Position::Root,
        }
    }

    fn finish(&mut self) -> sval::Result {
        if self.map == 0 && self.seq == 0 {
            Ok(())
        } else {
            Err(sval::Error)
        }
    }
}

fn number(src: &[u8], mut head: usize) -> sval::Result<(&str, usize)> {
    let start = head;

    // TODO: Proper number parser
    while head < src.len() {
        match src[head] {
            b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' => {
                head += 1;
            }
            _ => break,
        }
    }

    let str = core::str::from_utf8(&src[start..head])?;

    Ok((str, head))
}

fn str_fragment(src: &[u8], mut head: usize) -> sval::Result<(&str, bool, usize)> {
    let start = head;

    if src[head] == b'\\' {
        head += 1;

        match src[head] {
            b'n' => return Ok(("\n", true, head + 1)),
            b'r' => return Ok(("\r", true, head + 1)),
            b'"' => return Ok(("\"", true, head + 1)),
            b'\\' => return Ok(("\\", true, head + 1)),
            _ => todo!(),
        }
    }

    // Scan through the input until we reach the end or an escaped character
    let mut partial = false;
    while head < src.len() {
        match src[head] {
            b'\\' => {
                partial = true;
                break;
            }
            b'"' => break,
            _ => {
                head += 1;
            }
        }
    }

    let str = core::str::from_utf8(&src[start..head])?;

    Ok((str, partial, if partial { head } else { head + 1 }))
}
