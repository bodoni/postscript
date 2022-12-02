//! Parser for PostScript fonts.

#[macro_use]
extern crate typeface;

pub mod compact1;
pub mod type2;

pub use typeface::{Error, Result, Tape, Value, Walue};
