use core::str;
use corosensei::{stack::DefaultStack, CoroutineResult, ScopedCoroutine};

pub struct JsonSliceCoReader<'a, S> {
    state: State<'a, S>,
    resume: Resume<'a, S>,
}

type Resume<'a, S> = ScopedCoroutine<'a, State<'a, S>, State<'a, S>, sval::Result, DefaultStack>;
type Yielder<'a, S> = corosensei::Yielder<State<'a, S>, State<'a, S>>;

struct State<'a, S> {
    json: &'a [u8],
    stream: Option<S>,
}

impl<'a, S> State<'a, S> {
    fn step(
        &mut self,
        yielder: &Yielder<'a, S>,
        f: impl FnOnce(&mut S) -> sval::Result,
    ) -> sval::Result<Self> {
        let mut stream = self.stream.take().ok_or(sval::Error::unsupported())?;

        f(&mut stream)?;

        Ok(yielder.suspend(State {
            json: self.json,
            stream: Some(stream),
        }))
    }

    fn has_bytes(&self) -> bool {
        self.json.len() > 0
    }

    fn current_byte(&self) -> u8 {
        self.json[0]
    }

    fn next_byte(&mut self) -> u8 {
        let b = self.json[0];
        self.json = &self.json[1..];

        b
    }

    fn bytes(&self) -> &'a [u8] {
        self.json
    }
}

impl<'a, S: sval::Stream<'a>> JsonSliceCoReader<'a, S> {
    pub fn begin(json: &'a [u8], stream: S) -> Self {
        JsonSliceCoReader {
            state: State {
                json,
                stream: Some(stream),
            },
            resume: ScopedCoroutine::new(|yielder, state| interest(yielder, state)),
        }
    }

    pub fn resume(&mut self) -> sval::Result<bool> {
        match self.resume.resume(State {
            json: self.state.json,
            stream: self.state.stream.take(),
        }) {
            CoroutineResult::Yield(state) => {
                self.state = state;
                Ok(true)
            }
            CoroutineResult::Return(r) => r.map(|_| false),
        }
    }
}

fn interest<'a, S: sval::Stream<'a>>(
    yielder: &Yielder<'a, S>,
    mut state: State<'a, S>,
) -> sval::Result {
    while state.has_bytes() {
        match state.next_byte() {
            // Read a string
            b'"' => {
                state = state.step(yielder, |stream| stream.text_begin(None))?;
                state = interest_text(yielder, state)?;
            }
            _ => (),
        }
    }

    Ok(())
}

fn interest_text<'a, S: sval::Stream<'a>>(
    yielder: &Yielder<'a, S>,
    mut state: State<'a, S>,
) -> sval::Result<State<'a, S>> {
    let mut current_fragment = state.bytes();

    while state.has_bytes() {
        match state.current_byte() {
            // Yield an escaped character
            b'\\' => {
                // Get the text up to the escape sequence
                let fragment = str::from_utf8(
                    &current_fragment[..current_fragment.len() - state.bytes().len()],
                )?;
                state.next_byte();

                // Get the unescaped sequence
                let unescaped = match state.next_byte() {
                    b'\\' => "\\",
                    b'n' => "\n",
                    _ => return sval::result::unsupported(),
                };

                // Yield the fragment and the unescaped content
                state = state.step(yielder, |stream| {
                    stream.text_fragment(fragment)?;
                    stream.text_fragment(unescaped)
                })?;

                current_fragment = state.bytes();
            }
            // End a string
            b'"' => {
                // Get the text up to the end of the string
                let fragment = str::from_utf8(
                    &current_fragment[..current_fragment.len() - state.bytes().len()],
                )?;
                state.next_byte();

                // Yield the text and the end of the string
                state = state.step(yielder, |stream| {
                    stream.text_fragment(fragment)?;
                    stream.text_end()
                })?;

                return Ok(state);
            }
            _ => {
                state.next_byte();
            }
        }
    }

    sval::result::unsupported()
}
