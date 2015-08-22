//! The compact font format.

#![allow(non_snake_case)]

use std::io::{Cursor, Read, Seek};

use Result;
use band::{Band, ParametrizedValue, Value};

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
    pub private_dictionaries: Vec<Operations>,
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
        let mut private_dictionaries = vec![];
        for i in 0..(dictionaries.count as usize) {
            let dictionary = try!(dictionaries.get(i)).unwrap();
            encodings.push(try!(read_encoding(band, &dictionary)));
            charstrings.push(try!(read_charstrings(band, &dictionary)));
            charsets.push(try!(read_charset(band, &dictionary, charstrings[i].count as usize)));
            private_dictionaries.push(try!(read_private_dictionary(band, &dictionary)));
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
            private_dictionaries: private_dictionaries,
        })
    }
}

macro_rules! get_single(
    ($operations:expr, $operator:ident) => ({
        match $operations.get_single(Operator::$operator) {
            Some(Number::Integer(value)) => value,
            _ => raise!("failed to process an operation ({})", stringify!($operator)),
        }
    });
);

macro_rules! get_double(
    ($operations:expr, $operator:ident) => (
        match $operations.get_double(Operator::$operator) {
            Some((Number::Integer(value0), Number::Integer(value1))) => (value0, value1),
            _ => raise!("failed to process an operation ({})", stringify!($operator)),
        }
    );
);

fn read_encoding<T: Band>(_: &mut T, operations: &Operations) -> Result<Encoding> {
    Ok(match get_single!(operations, Encoding) {
        0 => Encoding::Standard,
        1 => Encoding::Expert,
        _ => unimplemented!(),
    })
}

fn read_charset<T: Band>(band: &mut T, operations: &Operations, glyphs: usize) -> Result<Charset> {
    match get_single!(operations, Charset) {
        0 => Ok(Charset::ISOAdobe),
        1 => Ok(Charset::Expert),
        2 => Ok(Charset::ExpertSubset),
        offset => {
            try!(band.jump(offset as u64));
            ParametrizedValue::read(band, glyphs)
        },
    }
}

fn read_charstrings<T: Band>(band: &mut T, operations: &Operations) -> Result<CharstringIndex> {
    try!(band.jump(get_single!(operations, Charstrings) as u64));
    ParametrizedValue::read(band, get_single!(operations, CharstringType))
}

fn read_private_dictionary<T: Band>(band: &mut T, operations: &Operations) -> Result<Operations> {
    let (size, offset) = get_double!(operations, Private);
    try!(band.jump(offset as u64));
    let chunk: Vec<u8> = try!(ParametrizedValue::read(band, size as usize));
    let mut band = Cursor::new(chunk);
    Value::read(&mut band)
}

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{CharstringIndex, DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
use self::compound::{Operator, Operations};
use self::primitive::Number;
