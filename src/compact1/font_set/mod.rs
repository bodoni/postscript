//! The font sets.

use crate::compact1::index::{CharStrings, Dictionaries, Names, Strings, Subroutines};
use crate::compact1::{CharSet, Encoding, Header, Operations, Operator};
use crate::{Result, Tape, Value, Walue};

macro_rules! get(
    (@single $operations:expr, $operator:ident) => (
        match $operations.get_single(crate::compact1::Operator::$operator) {
            Some(crate::compact1::Number::Integer(value)) => value,
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("failed to find an operation with operator ", stringify!($operator))),
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
            _ => raise!(concat!("failed to find an operation with operator ", stringify!($operator))),
        }
    );
);

pub mod character_id_keyed;
pub mod character_name_keyed;

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
    CharacterIDKeyed(character_id_keyed::Record),
    CharacterNameKeyed(character_name_keyed::Record),
}

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
                    tape.take_given(char_strings[i].count as usize)?
                }
            });
            records.push(tape.take_given((position, dictionary, &char_strings[i]))?);
        }
        Ok(Self {
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
    type Parameter = (u64, &'l Operations, &'l CharStrings);

    fn read<T: Tape>(
        tape: &mut T,
        (position, dictionary, char_strings): Self::Parameter,
    ) -> Result<Self> {
        if let Some((Operator::ROS, _)) = <[_]>::get(dictionary, 0) {
            Ok(Record::CharacterIDKeyed(tape.take_given((
                position,
                dictionary,
                char_strings,
            ))?))
        } else {
            Ok(Record::CharacterNameKeyed(
                tape.take_given((position, dictionary))?,
            ))
        }
    }
}
