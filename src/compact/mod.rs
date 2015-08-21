//! The compact font format.

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
        let strings = try!(StringIndex::read(band));
        let subroutines = try!(SubroutineIndex::read(band));

        let mut encodings = vec![];
        let mut char_sets = vec![];
        let mut char_strings = vec![];
        for i in 0..(dictionaries.count as usize) {
            let dictionary = try!(dictionaries.get(i)).unwrap();
            encodings.push(try!(read_encoding(band, &dictionary)));
            char_strings.push(try!(read_char_strings(band, &dictionary)));
            char_sets.push(try!(read_char_set(band, &dictionary, char_strings[i].count as usize)));
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

macro_rules! operand(
    ($operation:expr, $operand:ident) => (
        match $operation {
            Some(operands) => match (operands.len(), operands.get(0)) {
                (1, Some(&Operand::$operand(value))) => Some(value),
                _ => None,
            },
            _ => None,
        }
    );
    ($operation:expr) => (operand!($operation, Integer));
);

fn read_encoding<T: Band>(_: &mut T, operations: &Operations) -> Result<Encoding> {
    Ok(match operand!(operations.get(Operator::Encoding)) {
        Some(0) => Encoding::Standard,
        Some(1) => Encoding::Expert,
        Some(_) => unimplemented!(),
        _ => raise!("failed to process an operation"),
    })
}

fn read_char_set<T: Band>(band: &mut T, operations: &Operations, glyphs: usize)
                          -> Result<CharSet> {

    Ok(match operand!(operations.get(Operator::charset)) {
        Some(0) => CharSet::ISOAdobe,
        Some(1) => CharSet::Expert,
        Some(2) => CharSet::ExpertSubset,
        Some(offset) => {
            try!(band.jump(offset as u64));
            try!(CharSet::read(band, glyphs))
        },
        _ => raise!("failed to process an operation"),
    })
}

fn read_char_strings<T: Band>(band: &mut T, operations: &Operations)
                              -> Result<CharStringIndex> {

    let offset = match operand!(operations.get(Operator::CharStrings)) {
        Some(offset) => offset as u64,
        _ => raise!("failed to process an operation"),
    };
    let kind = match operand!(operations.get(Operator::CharstringType)) {
        Some(kind) => kind,
        _ => raise!("failed to process an operation"),
    };
    try!(band.jump(offset as u64));
    Ok(try!(CharStringIndex::read(band, kind)))
}

pub mod compound;
pub mod primitive;

use self::compound::{CharSet, Encoding, Header};
use self::compound::{CharStringIndex, DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
use self::compound::{Operator, Operand, Operations};
