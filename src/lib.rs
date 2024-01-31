//! Parser of PostScript fonts.

#[macro_use(dereference, jump_take, jump_take_given, raise, table)]
extern crate typeface;

pub mod compact1;
pub mod type1;
pub mod type2;

pub use typeface::{tape, value, walue, Error, Result};
