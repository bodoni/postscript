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
    pub charsets: Vec<Charset>,
    pub charstrings: Vec<CharstringIndex>,
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
        let mut charsets = vec![];
        let mut charstrings = vec![];
        for i in 0..(dictionaries.count as usize) {
            let dictionary = try!(dictionaries.get(i)).unwrap();
            encodings.push(try!(read_encoding(band, &dictionary)));
            charstrings.push(try!(read_charstrings(band, &dictionary)));
            charsets.push(try!(read_charset(band, &dictionary, charstrings[i].count as usize)));
        }

        Ok(FontSet {
            header: header,
            names: names,
            dictionaries: dictionaries,
            strings: strings,
            subroutines: subroutines,
            encodings: encodings,
            charsets: charsets,
            charstrings: charstrings,
        })
    }
}

macro_rules! argument(
    ($operation:expr, $argument:ident) => (
        match $operation {
            Some(arguments) => match (arguments.len(), arguments.get(0)) {
                (1, Some(&Number::$argument(value))) => Some(value),
                _ => None,
            },
            _ => None,
        }
    );
    ($operation:expr) => (argument!($operation, Integer));
);

fn read_encoding<T: Band>(_: &mut T, operations: &Operations) -> Result<Encoding> {
    Ok(match argument!(operations.get(Operator::Encoding)) {
        Some(0) => Encoding::Standard,
        Some(1) => Encoding::Expert,
        Some(_) => unimplemented!(),
        _ => raise!("failed to process an operation"),
    })
}

fn read_charset<T: Band>(band: &mut T, operations: &Operations, glyphs: usize)
                          -> Result<Charset> {

    Ok(match argument!(operations.get(Operator::charset)) {
        Some(0) => Charset::ISOAdobe,
        Some(1) => Charset::Expert,
        Some(2) => Charset::ExpertSubset,
        Some(offset) => {
            try!(band.jump(offset as u64));
            try!(Charset::read(band, glyphs))
        },
        _ => raise!("failed to process an operation"),
    })
}

fn read_charstrings<T: Band>(band: &mut T, operations: &Operations)
                             -> Result<CharstringIndex> {

    let offset = match argument!(operations.get(Operator::CharStrings)) {
        Some(offset) => offset as u64,
        _ => raise!("failed to process an operation"),
    };
    let format = match argument!(operations.get(Operator::CharstringType)) {
        Some(format) => format,
        _ => raise!("failed to process an operation"),
    };
    try!(band.jump(offset as u64));
    Ok(try!(CharstringIndex::read(band, format)))
}

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{CharstringIndex, DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
use self::compound::{Operator, Operations};
use self::primitive::Number;
