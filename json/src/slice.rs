use core::cmp::Ordering;

pub struct JsonBufReader<'a> {
    src: &'a [u8],
    head: usize,
    stack: Stack,
    position: Position,
}

impl<'a> sval::Source<'a> for JsonBufReader<'a> {
    fn stream_resume<'b, R: sval::Receiver<'b>>(
        &mut self,
        mut receiver: R,
    ) -> sval::Result<sval::Resume>
    where
        'a: 'b,
    {
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

                        self.maybe_done()
                    }
                    // If the string has escapes then yield this fragment
                    // The next time we loop through we'll grab the next one
                    else {
                        todo!()
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

                    self.map_end(receiver)?;

                    return self.maybe_done();
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

                    self.seq_end(receiver)?;

                    return self.maybe_done();
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

                    self.map_value_seq_elem_end(receiver)?;

                    return Ok(sval::Resume::Continue);
                }
                // The boolean value `true`
                b't' => {
                    if let Some(b"true") = self.src.get(self.head..self.head + 4) {
                        self.head += 4;

                        self.value_begin(&mut receiver)?;

                        receiver.bool(true)?;

                        return self.maybe_done();
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

                        return self.maybe_done();
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

                        return self.maybe_done();
                    } else {
                        todo!()
                    }
                }
                // Whitespace
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.head += 1;
                }
                // TODO: Numbers
                // NOTE: For big numbers we'll need to know whether we're looking at a human readable format or not
                // If it's not, then we need to convert it into its binary representation
                _ => todo!(),
            }
        }

        self.maybe_done()
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

impl<'a> JsonBufReader<'a> {
    pub fn new(src: &'a str) -> JsonBufReader<'a> {
        JsonBufReader {
            src: src.as_bytes(),
            head: 0,
            stack: Stack::new(),
            position: Position::Root,
        }
    }

    fn map_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => receiver.seq_elem_begin()?,
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
            Position::SeqEmpty | Position::SeqElem => receiver.seq_elem_begin()?,
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
                receiver.seq_elem_end()?;
            }
            _ => todo!(),
        }

        self.stack.pop_seq()?;
        self.position = self.stack.position();

        receiver.seq_end()
    }

    fn map_value_seq_elem_end<'b>(
        &mut self,
        mut receiver: impl sval::Receiver<'b>,
    ) -> sval::Result {
        match self.position {
            Position::SeqElem => receiver.seq_elem_end(),
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

                receiver.seq_elem_begin()
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

                receiver.seq_elem_begin()
            }
            Position::MapValue => receiver.map_value_begin(),
            Position::Root => Ok(()),
            _ => todo!(),
        }
    }

    fn maybe_done(&mut self) -> sval::Result<sval::Resume> {
        if self.head < self.src.len() {
            Ok(sval::Resume::Continue)
        } else {
            self.stack.finish()?;

            Ok(sval::Resume::Done)
        }
    }
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

fn str_fragment(src: &[u8], mut head: usize) -> sval::Result<(&str, bool, usize)> {
    let start = head;

    if src[head] == b'\\' {
        // Handle unescaping this next character
        todo!()
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
