//! The character-ID-keyed fonts.

use std::io::Cursor;

use crate::compact1::index::{CharacterStrings, Dictionaries, Subroutines};
use crate::compact1::{GlyphID, Number, Operations, Operator, StringID};
use crate::Result;

/// A character-ID-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct Record {
    pub registry: StringID,
    pub ordering: StringID,
    pub supplement: Number,
    pub encoding: Encoding,
    pub operations: Vec<Operations>,
    pub records: Vec<RecordInner>,
}

/// A record in a character-ID-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct RecordInner {
    pub operations: Operations,
    pub subroutines: Subroutines,
}

/// An encoding of a glyph-to-dictionary mapping.
#[derive(Clone, Debug)]
pub enum Encoding {
    /// Format 0.
    Format0(Encoding0),
    /// Format 3.
    Format3(Encoding3),
}

/// A glyph-to-dictionary encoding in format 0.
#[derive(Clone, Debug)]
pub struct Encoding0 {
    pub format: u8,              // format
    pub dictionary_ids: Vec<u8>, // fds
}

table! {
    /// A glyph-to-dictionary encoding in format 3.
    pub Encoding3 {
        format      (u8 ) = { 3 }, // format
        range_count (u16), // nRanges

        ranges (Vec<Range3>) |this, tape| { // Range3
            tape.take_given(this.range_count as usize)
        },

        glyph_count (u16), // sentinel
    }
}

table! {
    /// A range of a glyph-to-dictionary encoding in format 3.
    #[derive(Copy)]
    pub Range3 {
        first_glyph_id (GlyphID), // first
        dictionary_id  (u8     ), // fd
    }
}

impl<'l> crate::walue::Read<'l> for Record {
    type Parameter = (u64, &'l Operations, &'l CharacterStrings);

    fn read<T: crate::tape::Read>(
        tape: &mut T,
        (position, top_operations, character_strings): Self::Parameter,
    ) -> Result<Self> {
        let operands = match top_operations.get(Operator::ROS) {
            Some(operands) if operands.len() == 3 => operands,
            _ => raise!("found a malformed character-ID-keyed record"),
        };
        let offset = get!(@single top_operations, FDSelect);
        let encoding = jump_take_given!(@unwrap tape, position, offset, character_strings);
        let offset = get!(@single top_operations, FDArray);
        let operations: Dictionaries = jump_take!(@unwrap tape, position, offset);
        let operations: Vec<_> = (&operations).try_into()?;
        let mut records = vec![];
        for top_operations in operations.iter() {
            records.push(tape.take_given((position, top_operations))?);
        }
        Ok(Self {
            registry: operands[0].try_into()?,
            ordering: operands[1].try_into()?,
            supplement: operands[2],
            encoding,
            operations,
            records,
        })
    }
}

impl<'l> crate::walue::Read<'l> for RecordInner {
    type Parameter = (u64, &'l Operations);

    fn read<T: crate::tape::Read>(
        tape: &mut T,
        (position, top_operations): Self::Parameter,
    ) -> Result<Self> {
        use crate::tape::Read;

        let (size, offset) = get!(@double top_operations, Private);
        let chunk: Vec<u8> = jump_take_given!(@unwrap tape, position, offset, size as usize);
        let operations = Cursor::new(chunk).take::<Operations>()?;
        let subroutines = match get!(@try @single operations, Subrs) {
            Some(another_offset) => jump_take!(@unwrap tape, position, offset + another_offset),
            _ => Default::default(),
        };
        Ok(Self {
            operations,
            subroutines,
        })
    }
}

impl<'l> crate::walue::Read<'l> for Encoding {
    type Parameter = &'l CharacterStrings;

    fn read<T: crate::tape::Read>(
        tape: &mut T,
        character_strings: Self::Parameter,
    ) -> Result<Self> {
        Ok(match tape.peek::<u8>()? {
            0 => Encoding::Format0(tape.take_given(character_strings)?),
            3 => Encoding::Format3(tape.take()?),
            format => {
                raise!("found an unknown format of the glyph-to-dictionary encoding ({format})")
            }
        })
    }
}

impl<'l> crate::walue::Read<'l> for Encoding0 {
    type Parameter = &'l CharacterStrings;

    fn read<T: crate::tape::Read>(
        tape: &mut T,
        character_strings: Self::Parameter,
    ) -> Result<Self> {
        let format = tape.take()?;
        debug_assert_eq!(format, 0);
        Ok(Self {
            format,
            dictionary_ids: tape.take_given(character_strings.count as usize)?,
        })
    }
}
