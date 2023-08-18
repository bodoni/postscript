//! The character-name-keyed fonts.

use std::io::Cursor;

use crate::compact1::index::Subroutines;
use crate::compact1::Operations;
use crate::{Result, Tape, Walue};

/// A character-name-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct Record {
    pub operations: Operations,
    pub subroutines: Subroutines,
}

impl<'l> Walue<'l> for Record {
    type Parameter = (u64, &'l Operations);

    fn read<T: Tape>(tape: &mut T, (position, top_operations): Self::Parameter) -> Result<Self> {
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
