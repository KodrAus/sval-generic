#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::io::{self, Read},
        Result, Stream,
    };

    pub fn stream_read<'sval, R: Read>(
        stream: &mut (impl Stream<'sval> + ?Sized),
        mut read: R,
        buf: &mut [u8],
    ) -> Result {
        assert_ne!(0, buf.len(), "attempt to read into zero-length buffer");

        stream.binary_begin(None)?;

        loop {
            match read.read(buf) {
                Ok(0) => break,
                Ok(n) => stream.binary_fragment_computed(&buf[..n])?,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }

        stream.binary_end()
    }
}

#[cfg(feature = "std")]
pub use self::std_support::*;
