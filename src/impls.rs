use crate::value::{self, Value};

impl Value for i128 {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i128(*self)
    }

    fn stream_for_all<'a, S>(&self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.i128(*self)
    }
}

impl Value for str {
    fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
    where
        S: value::Stream<'a>,
    {
        stream.str(self)
    }

    fn to_str(&self) -> Option<&str> {
        Some(self)
    }
}
