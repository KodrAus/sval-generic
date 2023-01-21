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
            let mut buf = v.as_bytes();

            while buf.len() > 0 {
                match self.io.write(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        buf = &buf[n..];
                    }
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        self.result = Err(e);
                        return Err(fmt::Error);
                    }
                }
            }

            Ok(())
        }
    }

    let mut io = IoToFmt { io, result: Ok(()) };

    let _ = crate::stream_to_fmt(&mut io, v);

    io.result
}
