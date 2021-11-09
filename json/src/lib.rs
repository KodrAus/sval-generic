mod fmt;
pub use self::fmt::{to_fmt, Formatter};

mod std_support;
pub use self::std_support::to_string;

// TODO: Write JSON parsers:
// - `impl AsRef<[u8]>` as `impl Value`
// - `impl Read` as `impl Source`
