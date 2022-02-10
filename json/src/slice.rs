pub struct JsonSlice<'a> {
    src: &'a [u8],
    in_string: bool,
    stack: Stack,
    context: Position,
}

impl<'a> JsonSlice<'a> {
    pub fn new(src: &'a str) -> JsonSlice<'a> {
        JsonSlice {
            src: src.as_bytes(),
            in_string: false,
            stack: Stack::new(),
            context: Position::Root,
        }
    }

    fn map_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.context {
            Position::Seq => receiver.seq_elem_begin()?,
            Position::MapValue => receiver.map_value_begin()?,
            Position::Root => (),
            _ => todo!(),
        }

        self.stack.push_map()?;
        self.context = Position::MapEmpty;

        receiver.map_begin(None)
    }

    fn map_key_end<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        if !matches!(self.context, Position::MapKey) {
            todo!();
        }

        self.context = Position::MapValue;

        receiver.map_key_end()
    }

    fn map_end<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.context {
            Position::MapEmpty => (),
            Position::MapValue => {
                receiver.map_value_end()?;
            }
            _ => todo!(),
        }

        self.stack.pop_map()?;
        self.context = detect_context(self.src);

        receiver.map_end()
    }

    fn seq_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.context {
            Position::Seq => receiver.seq_elem_begin()?,
            Position::MapValue => receiver.map_value_begin()?,
            Position::Root => (),
            _ => todo!(),
        }

        self.stack.push_seq()?;
        self.context = Position::Seq;

        receiver.seq_begin(None)
    }

    fn seq_end<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result
    where
        'a: 'b,
    {
        if !matches!(self.context, Position::Seq) {
            todo!()
        }

        self.stack.pop_seq()?;
        self.context = detect_context(self.src);

        receiver.seq_end()
    }

    fn map_value_seq_elem_end<'b>(
        &mut self,
        mut receiver: impl sval::Receiver<'b>,
    ) -> sval::Result {
        match self.context {
            Position::Seq => receiver.seq_elem_end(),
            Position::MapValue => {
                self.context = Position::MapKey;

                receiver.map_value_end()
            }
            _ => todo!(),
        }
    }

    fn str_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.context {
            Position::Seq => receiver.seq_elem_begin(),
            Position::MapEmpty => {
                self.context = Position::MapKey;

                receiver.map_key_begin()
            }
            Position::MapKey => receiver.map_key_begin(),
            Position::MapValue => receiver.map_value_begin(),
            Position::Root => Ok(()),
        }
    }

    fn value_begin<'b>(&mut self, mut receiver: impl sval::Receiver<'b>) -> sval::Result {
        match self.context {
            Position::Seq => receiver.seq_elem_begin(),
            Position::MapValue => receiver.map_value_begin(),
            Position::Root => Ok(()),
            _ => todo!(),
        }
    }

    fn maybe_done(&mut self) -> sval::Result<sval::Resume> {
        if self.src.len() > 0 {
            Ok(sval::Resume::Continue)
        } else {
            self.stack.finish()?;

            Ok(sval::Resume::Done)
        }
    }
}

#[derive(Debug)]
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

    fn finish(&mut self) -> sval::Result {
        if self.map == 0 && self.seq == 0 {
            Ok(())
        } else {
            Err(sval::Error)
        }
    }
}

enum Position {
    Root,
    MapEmpty,
    MapKey,
    MapValue,
    Seq,
}

impl<'a> sval::Source<'a> for JsonSlice<'a> {
    fn stream_resume<'b, R: sval::Receiver<'b>>(
        &mut self,
        mut receiver: R,
    ) -> sval::Result<sval::Resume>
    where
        'a: 'b,
    {
        if self.in_string {
            let (fragment, partial) = str_fragment(self.src)?;

            todo!()
        } else {
            'strip_whitespace: loop {
                match self.src.get(0) {
                    // Start a map
                    Some(b'{') => {
                        self.src = &self.src[1..];

                        self.map_begin(receiver)?;

                        return Ok(sval::Resume::Continue);
                    }
                    // Begin a seq
                    Some(b'[') => {
                        self.src = &self.src[1..];

                        self.seq_begin(receiver)?;

                        return Ok(sval::Resume::Continue);
                    }
                    // End a map
                    Some(b'}') => {
                        self.src = &self.src[1..];

                        self.map_end(receiver)?;

                        return self.maybe_done();
                    }
                    // End a seq
                    Some(b']') => {
                        self.src = &self.src[1..];

                        self.seq_end(receiver)?;

                        return self.maybe_done();
                    }
                    // End a map key
                    Some(b':') => {
                        self.src = &self.src[1..];

                        self.map_key_end(receiver)?;

                        return Ok(sval::Resume::Continue);
                    }
                    // End either a map value or seq elem
                    Some(b',') => {
                        self.src = &self.src[1..];

                        self.map_value_seq_elem_end(receiver)?;

                        return Ok(sval::Resume::Continue);
                    }
                    // Begin a string
                    Some(b'"') => {
                        self.src = &self.src[1..];

                        self.str_begin(&mut receiver)?;

                        let (fragment, partial) = str_fragment(self.src)?;

                        // If the string is complete (with no escapes)
                        // then we can yield it directly
                        return if !partial {
                            self.src = &self.src[fragment.len() + 1..];

                            receiver.str(fragment)?;

                            return self.maybe_done();
                        }
                        // If the string has escapes then yield this fragment
                        // The next time we loop through we'll grab the next one
                        else {
                            self.src = &self.src[fragment.len()..];

                            self.in_string = true;

                            receiver.text_begin(None)?;
                            receiver.text_fragment(fragment)?;

                            Ok(sval::Resume::Continue)
                        };
                    }
                    // The boolean value `true`
                    Some(b't') => {
                        if let Some(b"true") = self.src.get(0..4) {
                            self.src = &self.src[4..];

                            self.value_begin(&mut receiver)?;

                            receiver.bool(true)?;
                        } else {
                            todo!()
                        }

                        return self.maybe_done();
                    }
                    // The boolean value `false`
                    Some(b'f') => {
                        if let Some(b"false") = self.src.get(0..5) {
                            self.src = &self.src[5..];

                            self.value_begin(&mut receiver)?;

                            receiver.bool(false)?;
                        } else {
                            todo!()
                        }

                        return self.maybe_done();
                    }
                    // The value `null`
                    Some(b'n') => {
                        if let Some(b"null") = self.src.get(0..4) {
                            self.src = &self.src[4..];

                            self.value_begin(&mut receiver)?;

                            receiver.null()?;
                        } else {
                            todo!()
                        }

                        return self.maybe_done();
                    }
                    Some(b' ') => {
                        self.src = &self.src[1..];
                        continue 'strip_whitespace;
                    }
                    // Numbers
                    Some(_) => todo!(),
                    None => return self.maybe_done(),
                }
            }
        }
    }
}

// The stack-free approach we use is to combine a strategy for
// quickly detecting whether we're in a map or seq with a strategy
// for detecting whether {} and [] are balanced

fn str_fragment(mut src: &[u8]) -> sval::Result<(&str, bool)> {
    let original = src;

    if src.len() == 0 {
        // EOF
        todo!()
    }

    if src[0] == b'\\' {
        // Handle unescaping this next character
        todo!()
    }

    // Scan through the input until we reach the end or an escaped character
    let mut partial = false;
    'str: while src.len() > 0 {
        match src[0] {
            b'\\' => {
                partial = true;
                break 'str;
            }
            b'"' => break 'str,
            _ => (),
        }

        // Skip over this character and continue
        src = &src[1..];
        continue 'str;
    }

    let len = original.len() - src.len();
    let str = core::str::from_utf8(&src[0..len])?;

    Ok((str, partial))
}

fn detect_context(mut src: &[u8]) -> Position {
    // Scan through the input to check whether we're in a map or sequence
    // This will need to scan ahead through at most a single string key

    while src.len() > 0 {
        // First, check if we're at the edge of a map or seq or in the middle of one
        match src[0] {
            // Ending a map means we must have been in a map
            b'}' => return Position::MapValue,
            // Ending a seq means we must have been in a seq
            b']' => return Position::Seq,
            // A comma could be either a map or a seq
            // The next item will tell us what we're looking at
            b',' => {
                src = &src[1..];
                return detect_context_from_key_or_elem(src);
            }
            b' ' => {
                src = &src[1..];
            }
            _ => todo!(),
        }
    }

    Position::Root
}

fn detect_context_from_key_or_elem(mut src: &[u8]) -> Position {
    while src.len() > 0 {
        match src[0] {
            // If we see any value except a string we must be in a seq
            b't' | b'f' | b'n' | b'{' | b'[' => return Position::Seq,
            // If we see a string then we could be in either a map or a seq
            // The next item will tell us what we're looking at
            b'"' => {
                src = &src[1..];
                let mut escaped = false;

                // Find the end of the string so we can check the character after it
                while src.len() > 0 {
                    match src[0] {
                        b'\\' => {
                            escaped = !escaped;
                            src = &src[1..];
                        }
                        b'"' if !escaped => break,
                        _ => {
                            src = &src[1..];
                        }
                    }
                }

                src = &src[1..];

                // TODO: If we had to parse a string here then cache its results

                while src.len() > 0 {
                    match src[0] {
                        // If we encounter a colon then we're in a map
                        b':' => return Position::MapValue,
                        // If we encounter a comma or seq end then we're in a seq
                        b']' | b',' => return Position::Seq,
                        b' ' => (),
                        _ => todo!(),
                    }

                    src = &src[1..];
                }
            }
            b' ' => {
                src = &src[1..];
            }
            _ => todo!(),
        }
    }

    // EOF
    todo!()
}
