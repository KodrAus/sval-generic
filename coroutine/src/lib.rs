#![feature(generic_associated_types)]

pub mod co;
mod impls;
pub mod value;

pub use sval::{data, Error, Receiver, Result};
