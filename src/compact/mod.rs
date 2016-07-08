//! The compact font format.

#[macro_use]
mod macros;

mod char_set;
mod encoding;
mod font_set;
mod header;
mod index;
mod number;
mod offset;
mod operation;
mod primitive;

pub use self::char_set::{CharSet, CharSet1, CharSetRange1};
pub use self::encoding::Encoding;
pub use self::font_set::FontSet;
pub use self::header::Header;
pub use self::index::{CharStrings, Index, Names};
pub use self::index::{Strings, Subroutines, TopDictionaries};
pub use self::number::Number;
pub use self::offset::Offset;
pub use self::operation::{Operation, Operations, Operator};

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;

/// An offset size.
pub type OffsetSize = u8;
