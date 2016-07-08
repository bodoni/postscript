//! The compact font format.

mod char_set;
mod encoding;
mod font_set;
mod header;
mod index;
mod number;
mod offset;
mod operation;

pub use self::char_set::{CharSet, CharSet1, Range1};
pub use self::encoding::Encoding;
pub use self::font_set::FontSet;
pub use self::header::Header;
pub use self::index::{CharStrings, Index, Names, Strings, Subroutines, TopDictionaries};
pub use self::number::Number;
pub use self::offset::{Offset, OffsetSize};
pub use self::operation::{Operation, Operations, Operator};

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;
