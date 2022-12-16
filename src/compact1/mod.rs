//! The [compact font format][1] of version 1.0.
//!
//! [1]: https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf

pub mod char_set;
pub mod encoding;
pub mod font_set;
pub mod index;

mod header;
mod number;
mod offset;
mod operation;

pub use char_set::CharSet;
pub use encoding::Encoding;
pub use font_set::FontSet;
pub use header::Header;
pub use index::Index;
pub use number::Number;
pub use offset::{Offset, OffsetSize};
pub use operation::{Operand, Operation, Operations, Operator};

use crate::{Error, Result};

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;

impl TryFrom<Number> for StringID {
    type Error = Error;

    #[inline]
    fn try_from(number: Number) -> Result<Self> {
        match number {
            Number::Integer(value) if value >= 0 => Ok(value as Self),
            _ => raise!("found a malformed string ID"),
        }
    }
}
