//! The [compact font format][1].
//!
//! [1]: https://www.adobe.com/content/dam/Adobe/en/devnet/font/pdfs/5176.CFF.pdf

mod encoding;
mod font_set;
mod header;
mod number;
mod offset;

pub mod char_set;
pub mod index;
pub mod operation;

pub use self::char_set::CharSet;
pub use self::encoding::Encoding;
pub use self::font_set::FontSet;
pub use self::header::Header;
pub use self::offset::{Offset, OffsetSize};
pub use self::operation::Operations;

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;
