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
        tape.jump(position + offset as u64)?;
        let chunk = tape.take_given::<Vec<u8>>(size as usize)?;
        let operations = Cursor::new(chunk).take::<Operations>()?;
        let subroutines = match get!(@try @single operations, Subrs) {
            Some(another_offset) => {
                tape.jump(position + offset as u64 + another_offset as u64)?;
                tape.take()?
            }
            _ => Default::default(),
        };
        Ok(Self {
            operations,
            subroutines,
        })
    }
}
