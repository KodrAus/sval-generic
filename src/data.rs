pub mod bytes;
pub mod digits;
pub mod error;
pub mod tag;

#[doc(inline)]
pub use self::{
    bytes::{bytes, Bytes},
    digits::{digits, digits_unchecked, Digits},
    error::Error,
    tag::{type_tag, variant_tag, TypeTag, VariantTag},
};

#[cfg(feature = "std")]
#[doc(inline)]
pub use self::error::error;
