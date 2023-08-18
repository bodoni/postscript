//! The font sets.

macro_rules! get(
    (@single $operations:expr, $operator:ident) => (
        match $operations.get_single(crate::compact1::Operator::$operator) {
            Some(crate::compact1::Number::Integer(value)) => value,
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("found no operation with operator ", stringify!($operator))),
        }
    );
    (@try @single $operations:expr, $operator:ident) => (
        match $operations.get_single(crate::compact1::Operator::$operator) {
            Some(crate::compact1::Number::Integer(value)) => Some(value),
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => None,
        }
    );
    (@double $operations:expr, $operator:ident) => (
        match $operations.get_double(crate::compact1::Operator::$operator) {
            Some((crate::compact1::Number::Integer(value0), crate::compact1::Number::Integer(value1))) => (value0, value1),
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("found no operation with operator ", stringify!($operator))),
        }
    );
);

pub mod character_id_keyed;
pub mod character_name_keyed;

use crate::compact1::index::{CharacterStrings, Dictionaries, Names, Strings, Subroutines};
use crate::compact1::{CharacterSet, Encoding, Header, Operations, Operator};
use crate::{Result, Tape, Value, Walue};

/// A font set.
#[derive(Clone, Debug)]
pub struct FontSet {
    pub header: Header,
    pub names: Names,
    pub operations: Vec<Operations>,
    pub strings: Strings,
    pub subroutines: Subroutines,
    pub encodings: Vec<Encoding>,
    pub character_strings: Vec<CharacterStrings>,
    pub character_sets: Vec<CharacterSet>,
    pub records: Vec<Record>,
}

/// A record in a font set.
#[derive(Clone, Debug)]
pub enum Record {
    CharacterIDKeyed(character_id_keyed::Record),
    CharacterNameKeyed(character_name_keyed::Record),
}

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let header = tape.take::<Header>()?;
        let names: Names = jump_take!(@unwrap tape, position, header.header_size);
        let operations: Vec<_> = (&tape.take::<Dictionaries>()?).try_into()?;
        let strings = tape.take::<Strings>()?;
        let subroutines = tape.take::<Subroutines>()?;
        let mut encodings = vec![];
        let mut character_sets = vec![];
        let mut character_strings = vec![];
        let mut records = vec![];
        for (i, operations) in operations.iter().enumerate() {
            character_strings.push({
                tape.jump(position + get!(@single operations, CharStrings) as u64)?;
                tape.take_given::<CharacterStrings>(get!(@single operations, CharStringType))?
            });
            character_sets.push(match get!(@single operations, CharSet) {
                0 => CharacterSet::ISOAdobe,
                1 => CharacterSet::Expert,
                2 => CharacterSet::ExpertSubset,
                offset => jump_take_given!(@unwrap tape, position, offset, character_strings[i].count as usize),
            });
            encodings.push(match get!(@single operations, Encoding) {
                0 => Encoding::Standard,
                1 => Encoding::Expert,
                offset => jump_take!(@unwrap tape, position, offset),
            });
            records.push(tape.take_given((position, operations, &character_strings[i]))?);
        }
        Ok(Self {
            header,
            names,
            operations,
            strings,
            subroutines,
            encodings,
            character_strings,
            character_sets,
            records,
        })
    }
}

impl<'l> Walue<'l> for Record {
    type Parameter = (u64, &'l Operations, &'l CharacterStrings);

    fn read<T: Tape>(
        tape: &mut T,
        (position, operations, character_strings): Self::Parameter,
    ) -> Result<Self> {
        if operations.contains_key(&Operator::ROS) {
            Ok(Record::CharacterIDKeyed(tape.take_given((
                position,
                operations,
                character_strings,
            ))?))
        } else {
            Ok(Record::CharacterNameKeyed(
                tape.take_given((position, operations))?,
            ))
        }
    }
}
