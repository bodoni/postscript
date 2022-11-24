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
    pub global_dictionaries: Vec<Operations>,
    pub global_subroutines: Subroutines,
    pub local_dictionaries: Vec<Operations>,
    pub local_subroutines: Vec<Subroutines>,
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
    (@try @double $operations:expr, $operator:ident) => (
        match $operations.get_double(Operator::$operator) {
            Some((value0, value1)) if is_i32!(value0) && is_i32!(value1) => {
                Some((value0 as i32, value1 as i32))
            },
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => None,
        }
    );
);

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let header = tape.take::<Header>()?;
        tape.jump(position + header.header_size as u64)?;
        let names = tape.take::<Names>()?.into()?;
        let global_dictionaries = tape.take::<Dictionaries>()?.into()?;
        let strings = tape.take::<Strings>()?;
        let global_subroutines = tape.take::<Subroutines>()?;
        let mut encodings = vec![];
        let mut char_sets = vec![];
        let mut char_strings = vec![];
        let mut local_dictionaries = vec![];
        let mut local_subroutines = vec![];
        for (i, dictionary) in global_dictionaries.iter().enumerate() {
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
            local_dictionaries.push({
                if cfg!(not(feature = "ignore-missing-operators")) {
                    let (size, offset) = get!(@double dictionary, Private);
                    tape.jump(position + offset as u64)?;
                    let chunk = tape.take_given::<Vec<u8>>(size as usize)?;
                    Cursor::new(chunk).take::<Operations>()?
                } else {
                    if let Some((size, offset)) = get!(@try @double dictionary, Private) {
                        tape.jump(position + offset as u64)?;
                        let chunk = tape.take_given::<Vec<u8>>(size as usize)?;
                        Cursor::new(chunk).take::<Operations>()?
                    } else {
                        Default::default()
                    }
                }
            });
            local_subroutines.push({
                if cfg!(not(feature = "ignore-missing-operators")) {
                    let (_, mut offset) = get!(@double dictionary, Private);
                    offset += get!(@single &local_dictionaries[i], Subrs);
                    tape.jump(position + offset as u64)?;
                    tape.take()?
                } else {
                    if let Some((_, mut offset)) = get!(@try @double dictionary, Private) {
                        offset += get!(@single &local_dictionaries[i], Subrs);
                        tape.jump(position + offset as u64)?;
                        tape.take()?
                    } else {
                        Default::default()
                    }
                }
            });
        }
        Ok(FontSet {
            header: header,
            names: names,
            strings: strings,
            encodings: encodings,
            char_sets: char_sets,
            char_strings: char_strings,
            global_dictionaries: global_dictionaries,
            global_subroutines: global_subroutines,
            local_dictionaries: local_dictionaries,
            local_subroutines: local_subroutines,
        })
    }
}
