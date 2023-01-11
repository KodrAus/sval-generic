use std::{
    fmt,
    io::{self, Write},
};

pub fn stream_to_writer(io: impl Write, v: impl sval::Value) -> io::Result<()> {
    struct IoToFmt<W> {
        io: W,
        result: io::Result<()>,
    }

    impl<W: Write> fmt::Write for IoToFmt<W> {
        fn write_str(&mut self, v: &str) -> fmt::Result {
            match self.0.write(v.as_bytes()) {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.result = Err(e);
                    Err(fmt::Error)
                }
            }
        }
    }

    let mut io = IoToFmt { io, result: Ok(()) };

    let _ = crate::stream_to_fmt(&mut io, v);

    io.result
}
