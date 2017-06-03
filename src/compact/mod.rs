//! The [compact font format][1].
//!
//! [1]: http://wwwimages.adobe.com/content/dam/Adobe/en/devnet/font/pdfs/5176.CFF.pdf

mod font_set;
mod header;
mod number;
mod offset;
mod operation;

pub mod char_set;
pub mod encoding;
pub mod index;

pub use self::char_set::CharSet;
pub use self::encoding::Encoding;
pub use self::font_set::FontSet;
pub use self::header::Header;
pub use self::index::Index;
pub use self::offset::{Offset, OffsetSize};
pub use self::operation::{Operand, Operation, Operations, Operator};

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;
