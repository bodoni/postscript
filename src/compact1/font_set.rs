//! The font sets.

use std::io::Cursor;

use crate::compact1::index::{CharStrings, Dictionaries, Names, Strings, Subroutines};
use crate::compact1::{CharSet, Encoding, Header, Number, Operations, Operator, StringID};
use crate::{Result, Tape, Value, Walue};

/// A font set.
#[derive(Clone, Debug)]
pub struct FontSet {
    pub header: Header,
    pub names: Vec<String>,
    pub strings: Strings,
    pub encodings: Vec<Encoding>,
    pub char_sets: Vec<CharSet>,
    pub char_strings: Vec<CharStrings>,
    pub operations: Vec<Operations>,
    pub subroutines: Subroutines,
    pub records: Vec<Record>,
}

/// A record in a font set.
#[derive(Clone, Debug)]
pub enum Record {
    CharacterIDKeyed(CharacterIDKeyedRecord),
    CharacterNameKeyed(CharacterNameKeyedRecord),
}

/// A character-ID-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct CharacterIDKeyedRecord {
    pub registry: StringID,
    pub ordering: StringID,
    pub supplement: Number,
    pub operations: Vec<Operations>,
    pub records: Vec<CharacterIDKeyedRecordRecord>,
}

/// A record in a character-ID-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct CharacterIDKeyedRecordRecord {
    pub operations: Operations,
    pub subroutines: Subroutines,
}

/// A character-name-keyed record in a font set.
#[derive(Clone, Debug)]
pub struct CharacterNameKeyedRecord {
    pub operations: Operations,
    pub subroutines: Subroutines,
}

macro_rules! get(
    (@single $operations:expr, $operator:ident) => (
        match $operations.get_single(Operator::$operator) {
            Some(Number::Integer(value)) => value,
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("failed to find an operation with operator ", stringify!($operator))),
        }
    );
    (@try @single $operations:expr, $operator:ident) => (
        match $operations.get_single(Operator::$operator) {
            Some(Number::Integer(value)) => Some(value),
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => None,
        }
    );
    (@double $operations:expr, $operator:ident) => (
        match $operations.get_double(Operator::$operator) {
            Some((Number::Integer(value0), Number::Integer(value1))) => (value0, value1),
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("failed to find an operation with operator ", stringify!($operator))),
        }
    );
);

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let header = tape.take::<Header>()?;
        tape.jump(position + header.header_size as u64)?;
        let names = tape.take::<Names>()?.into()?;
        let operations = tape.take::<Dictionaries>()?.into()?;
        let strings = tape.take::<Strings>()?;
        let subroutines = tape.take::<Subroutines>()?;
        let mut encodings = vec![];
        let mut char_sets = vec![];
        let mut char_strings = vec![];
        let mut records = vec![];
        for (i, dictionary) in operations.iter().enumerate() {
            encodings.push(match get!(@single dictionary, Encoding) {
                0 => Encoding::Standard,
                1 => Encoding::Expert,
                _ => unimplemented!(),
            });
            char_strings.push({
                tape.jump(position + get!(@single dictionary, CharStrings) as u64)?;
                tape.take_given::<CharStrings>(get!(@single dictionary, CharStringType))?
            });
            char_sets.push(match get!(@single dictionary, CharSet) {
                0 => CharSet::ISOAdobe,
                1 => CharSet::Expert,
                2 => CharSet::ExpertSubset,
                offset => {
                    tape.jump(position + offset as u64)?;
                    tape.take_given(char_strings[i].len())?
                }
            });
            records.push(tape.take_given((position, dictionary))?);
        }
        Ok(FontSet {
            header,
            names,
            strings,
            encodings,
            char_sets,
            char_strings,
            operations,
            subroutines,
            records,
        })
    }
}

impl<'l> Walue<'l> for Record {
    type Parameter = (u64, &'l Operations);

    fn read<T: Tape>(tape: &mut T, (position, dictionary): Self::Parameter) -> Result<Self> {
        if let Some((Operator::ROS, _)) = <[_]>::get(dictionary, 0) {
            Ok(Record::CharacterIDKeyed(
                tape.take_given((position, dictionary))?,
            ))
        } else {
            Ok(Record::CharacterNameKeyed(
                tape.take_given((position, dictionary))?,
            ))
        }
    }
}

impl<'l> Walue<'l> for CharacterIDKeyedRecord {
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
        Ok(CharacterIDKeyedRecord {
            registry: operands[0].try_into()?,
            ordering: operands[1].try_into()?,
            supplement: operands[2],
            operations: operations,
            records: records,
        })
    }
}

impl<'l> Walue<'l> for CharacterIDKeyedRecordRecord {
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
        Ok(CharacterIDKeyedRecordRecord {
            operations,
            subroutines,
        })
    }
}

impl<'l> Walue<'l> for CharacterNameKeyedRecord {
    type Parameter = (u64, &'l Operations);

    fn read<T: Tape>(tape: &mut T, (position, dictionary): Self::Parameter) -> Result<Self> {
        let (size, mut offset) = get!(@double dictionary, Private);
        tape.jump(position + offset as u64)?;
        let chunk = tape.take_given::<Vec<u8>>(size as usize)?;
        let operations = Cursor::new(chunk).take::<Operations>()?;
        offset += get!(@single operations, Subrs);
        tape.jump(position + offset as u64)?;
        let subroutines = tape.take()?;
        Ok(CharacterNameKeyedRecord {
            operations,
            subroutines,
        })
    }
}
