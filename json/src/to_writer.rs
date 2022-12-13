use std::{fmt, io::Write};

pub fn stream_to_writer(io: impl Write, v: impl sval::Value) -> sval::Result {
    struct IoToFmt<W>(W);

    impl<W: Write> fmt::Write for IoToFmt<W> {
        fn write_str(&mut self, v: &str) -> fmt::Result {
            self.0.write(v.as_bytes()).map_err(|_| fmt::Error)?;

            Ok(())
        }
    }

    crate::stream_to_fmt(IoToFmt(io), v)
}
