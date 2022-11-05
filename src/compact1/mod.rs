//! The [compact font format][1] of version 1.0.
//!
//! [1]: https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf

mod font_set;
mod header;
mod number;
mod offset;
mod operation;

pub mod char_set;
pub mod encoding;
pub mod index;

pub use char_set::CharSet;
pub use encoding::Encoding;
pub use font_set::FontSet;
pub use header::Header;
pub use index::Index;
pub use offset::{Offset, OffsetSize};
pub use operation::{Operand, Operation, Operations, Operator};

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;
