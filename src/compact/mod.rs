//! The compact font format.

#![allow(non_snake_case)]

use std::io::{Cursor, Read, Seek};

use Result;
use band::{Band, ParametrizedValue, Value};

/// A font set.
pub struct FontSet {
    pub header: Header,
    pub names: Vec<String>,
    pub top_dictionaries: Vec<Operations>,
    pub strings: Strings,
    pub subroutines: Subroutines,
    pub encodings: Vec<Encoding>,
    pub charsets: Vec<Charset>,
    pub charstrings: Vec<Charstrings>,
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
        let names = try!(try!(Names::read(band)).into_vec());
        let top_dictionaries = try!(try!(TopDictionaries::read(band)).into_vec());
        let strings = try!(Strings::read(band));
        let subroutines = try!(Subroutines::read(band));

        let mut encodings = vec![];
        let mut charsets = vec![];
        let mut charstrings = vec![];
        let mut private_dictionaries = vec![];
        for (i, dictionary) in top_dictionaries.iter().enumerate() {
            encodings.push(try!(read_encoding(band, dictionary)));
            charstrings.push(try!(read_charstrings(band, dictionary)));
            charsets.push(try!(read_charset(band, dictionary, charstrings[i].len())));
            private_dictionaries.push(try!(read_private_dictionary(band, dictionary)));
        }

        Ok(FontSet {
            header: header,
            names: names,
            top_dictionaries: top_dictionaries,
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

fn read_charstrings<T: Band>(band: &mut T, operations: &Operations) -> Result<Charstrings> {
    try!(band.jump(get_single!(operations, Charstrings) as u64));
    ParametrizedValue::read(band, get_single!(operations, CharstringType))
}

fn read_private_dictionary<T: Band>(band: &mut T, operations: &Operations) -> Result<Operations> {
    let (size, offset) = get_double!(operations, Private);
    try!(band.jump(offset as u64));
    let chunk: Vec<u8> = try!(ParametrizedValue::read(band, size as usize));
    Value::read(&mut Cursor::new(chunk))
}

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{Charstrings, Names, Strings, Subroutines, TopDictionaries};
use self::compound::{Operator, Operations};
use self::primitive::Number;
