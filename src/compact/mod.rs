//! Compact font format.

#![allow(non_snake_case)]

use std::io::{Read, Seek};

use Result;
use band::{Band, Value};

/// A font set.
pub struct FontSet {
    pub header: Header,
    pub names: NameIndex,
    pub dictionaries: DictionaryIndex,
    pub strings: StringIndex,
    pub subroutines: SubroutineIndex,
    pub encodings: Vec<Encoding>,
    pub char_sets: Vec<CharSet>,
    pub char_strings: Vec<CharStringIndex>,
}

impl FontSet {
    #[inline]
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Self> {
        Value::read(reader)
    }
}

impl Value for FontSet {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let header = try!(Header::read(band));
        try!(band.jump(header.hdrSize as u64));
        let names = try!(NameIndex::read(band));
        let dictionaries = try!(DictionaryIndex::read(band));
        if names.count != dictionaries.count {
            raise!("the name and top dictionary indices do not match");
        }
        let strings = try!(StringIndex::read(band));
        let subroutines = try!(SubroutineIndex::read(band));

        let mut encodings = vec![];
        let mut char_sets = vec![];
        let mut char_strings = vec![];
        for i in 0..(dictionaries.count as usize) {
            let dictionary = match try!(dictionaries.get(i)) {
                Some(dictionary) => dictionary,
                _ => raise!("failed to find a dictionary"),
            };
            encodings.push(try!(read_encoding(band, &dictionary)));
            char_sets.push(try!(read_char_set(band, &dictionary)));
            char_strings.push(try!(read_char_strings(band, &dictionary)));
        }

        Ok(FontSet {
            header: header,
            names: names,
            dictionaries: dictionaries,
            strings: strings,
            subroutines: subroutines,
            encodings: encodings,
            char_sets: char_sets,
            char_strings: char_strings,
        })
    }
}

macro_rules! read_operand(
    ($operations:ident, $operator:ident, $operand:ident) => (
        match $operations.get(Operator::$operator) {
            Some(operands) => match (operands.len(), operands.get(0)) {
                (1, Some(&Operand::$operand(value))) => value,
                _ => raise!("found an operator with invalid operands"),
            },
            _ => raise!("failed to find an operation"),
        }
    );
    ($operations:ident, $operator:ident) => (
        read_operand!($operations, $operator, Integer)
    );
);

fn read_encoding<T: Band>(_: &mut T, operations: &Operations) -> Result<Encoding> {
    Ok(match read_operand!(operations, Encoding) {
        0 => Encoding::Standard,
        1 => Encoding::Expert,
        _ => unimplemented!(),
    })
}

fn read_char_set<T: Band>(band: &mut T, operations: &Operations) -> Result<CharSet> {
    match read_operand!(operations, charset) {
        0 => Ok(CharSet::ISOAdobe),
        1 => Ok(CharSet::Expert),
        2 => Ok(CharSet::ExpertSubset),
        offset => {
            try!(band.jump(offset as u64));
            Value::read(band)
        }
    }
}

fn read_char_strings<T: Band>(band: &mut T, operations: &Operations) -> Result<CharStringIndex> {
    try!(band.jump(read_operand!(operations, CharStrings) as u64));
    Value::read(band)
}

pub mod compound;
pub mod primitive;

use self::compound::{CharSet, Encoding, Header};
use self::compound::{CharStringIndex, DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
use self::compound::{Operator, Operand, Operations};
