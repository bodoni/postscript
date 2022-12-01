//! The character-ID-keyed fonts.

use std::io::Cursor;

use crate::compact1::index::{Dictionaries, Subroutines};
use crate::compact1::{Number, Operations, Operator, StringID};
use crate::{Result, Tape, Walue};

/// A character-ID-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct Record {
    pub registry: StringID,
    pub ordering: StringID,
    pub supplement: Number,
    pub operations: Vec<Operations>,
    pub records: Vec<RecordInner>,
}

/// A record in a character-ID-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct RecordInner {
    pub operations: Operations,
    pub subroutines: Subroutines,
}

impl<'l> Walue<'l> for Record {
    type Parameter = (u64, &'l Operations);

    fn read<T: Tape>(tape: &mut T, (position, dictionary): Self::Parameter) -> Result<Self> {
        let operands = match <[_]>::get(dictionary, 0) {
            Some((Operator::ROS, operands)) if operands.len() == 3 => operands,
            _ => raise!("found a malformed character-ID-keyed font"),
        };
        let offset = get!(@single dictionary, FDArray);
        tape.jump(position + offset as u64)?;
        let operations = tape.take::<Dictionaries>()?.into()?;
        let mut records = vec![];
        for dictionary in operations.iter() {
            records.push(tape.take_given((position, dictionary))?);
        }
        Ok(Record {
            registry: operands[0].try_into()?,
            ordering: operands[1].try_into()?,
            supplement: operands[2],
            operations: operations,
            records: records,
        })
    }
}

impl<'l> Walue<'l> for RecordInner {
    type Parameter = (u64, &'l Operations);

    fn read<T: Tape>(tape: &mut T, (position, dictionary): Self::Parameter) -> Result<Self> {
        let (size, offset) = get!(@double dictionary, Private);
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
        Ok(RecordInner {
            operations,
            subroutines,
        })
    }
}
