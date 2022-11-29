//! The font sets.

use std::io::Cursor;

use crate::compact1::index::{CharStrings, Dictionaries, Names, Strings, Subroutines};
use crate::compact1::{CharSet, Encoding, Header, Operand, Operations, Operator};
use crate::{Result, Tape, Value};

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

/// A font record.
#[derive(Clone, Debug)]
pub enum Record {
    CharacterIDKeyed(CharacterIDKeyedRecord),
    CharacterNameKeyed(CharacterNameKeyedRecord),
}

/// A character-ID-keyed font record.
#[derive(Clone, Debug)]
pub struct CharacterIDKeyedRecord {
    pub operations: Vec<Operations>,
}

/// A character-name-keyed font record.
#[derive(Clone, Debug)]
pub struct CharacterNameKeyedRecord {
    pub operations: Operations,
    pub subroutines: Subroutines,
}

macro_rules! is_i32(($value:ident) => ($value as i32 as Operand == $value));

macro_rules! get(
    (@single $operations:expr, $operator:ident) => (
        match $operations.get_single(Operator::$operator) {
            Some(value) if is_i32!(value) => value as i32,
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("failed to find an operation with operator ", stringify!($operator))),
        }
    );
    (@double $operations:expr, $operator:ident) => (
        match $operations.get_double(Operator::$operator) {
            Some((value0, value1)) if is_i32!(value0) && is_i32!(value1) => {
                (value0 as i32, value1 as i32)
            },
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
            if let Some(Operator::ROS) = dictionary.ordering.get(0) {
                let offset = get!(@single dictionary, FDArray);
                tape.jump(position + offset as u64)?;
                let operations = tape.take::<Dictionaries>()?.into()?;
                records.push(Record::CharacterIDKeyed(CharacterIDKeyedRecord {
                    operations,
                }));
            } else {
                let (size, mut offset) = get!(@double dictionary, Private);
                tape.jump(position + offset as u64)?;
                let chunk = tape.take_given::<Vec<u8>>(size as usize)?;
                let operations = Cursor::new(chunk).take::<Operations>()?;
                offset += get!(@single operations, Subrs);
                tape.jump(position + offset as u64)?;
                let subroutines = tape.take()?;
                records.push(Record::CharacterNameKeyed(CharacterNameKeyedRecord {
                    operations,
                    subroutines,
                }));
            }
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
