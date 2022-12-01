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

    fn read<T: Tape>(tape: &mut T, (position, dictionary): Self::Parameter) -> Result<Self> {
        let (size, mut offset) = get!(@double dictionary, Private);
        tape.jump(position + offset as u64)?;
        let chunk = tape.take_given::<Vec<u8>>(size as usize)?;
        let operations = Cursor::new(chunk).take::<Operations>()?;
        offset += get!(@single operations, Subrs);
        tape.jump(position + offset as u64)?;
        let subroutines = tape.take()?;
        Ok(Self {
            operations,
            subroutines,
        })
    }
}
