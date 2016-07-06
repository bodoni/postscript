//! Parser for PostScript fonts.

#[macro_use]
mod macros;

mod tape;

pub mod compact;
pub mod type2;

pub use tape::{Tape, Value, Walue};

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;
