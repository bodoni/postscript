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
    pub global_subroutines: Subroutines,
    pub encodings: Vec<Encoding>,
    pub charsets: Vec<Charset>,
    pub charstrings: Vec<Charstrings>,
    pub private_dictionaries: Vec<Operations>,
    pub local_subroutines: Vec<Subroutines>,
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
        let global_subroutines = try!(Subroutines::read(band));

        let mut encodings = vec![];
        let mut charsets = vec![];
        let mut charstrings = vec![];
        let mut private_dictionaries = vec![];
        let mut local_subroutines = vec![];
        for (i, top) in top_dictionaries.iter().enumerate() {
            encodings.push(try!(read_encoding(band, top)));
            charstrings.push(try!(read_charstrings(band, top)));

            let glyphs = charstrings[i].len();
            charsets.push(try!(read_charset(band, top, glyphs)));

            private_dictionaries.push(try!(read_private_dictionary(band, top)));

            let private = &private_dictionaries[i];
            local_subroutines.push(try!(read_local_subroutines(band, top, private)));
        }

        Ok(FontSet {
            header: header,
            names: names,
            top_dictionaries: top_dictionaries,
            strings: strings,
            global_subroutines: global_subroutines,
            encodings: encodings,
            charsets: charsets,
            charstrings: charstrings,
            private_dictionaries: private_dictionaries,
            local_subroutines: local_subroutines,
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

fn read_encoding<T: Band>(_: &mut T, top: &Operations) -> Result<Encoding> {
    Ok(match get_single!(top, Encoding) {
        0 => Encoding::Standard,
        1 => Encoding::Expert,
        _ => unimplemented!(),
    })
}

fn read_charset<T: Band>(band: &mut T, top: &Operations, glyphs: usize) -> Result<Charset> {
    match get_single!(top, Charset) {
        0 => Ok(Charset::ISOAdobe),
        1 => Ok(Charset::Expert),
        2 => Ok(Charset::ExpertSubset),
        offset => {
            try!(band.jump(offset as u64));
            ParametrizedValue::read(band, glyphs)
        },
    }
}

fn read_charstrings<T: Band>(band: &mut T, top: &Operations) -> Result<Charstrings> {
    try!(band.jump(get_single!(top, Charstrings) as u64));
    ParametrizedValue::read(band, get_single!(top, CharstringType))
}

fn read_private_dictionary<T: Band>(band: &mut T, top: &Operations) -> Result<Operations> {
    let (size, offset) = get_double!(top, Private);
    try!(band.jump(offset as u64));
    let chunk: Vec<u8> = try!(ParametrizedValue::read(band, size as usize));
    Value::read(&mut Cursor::new(chunk))
}

fn read_local_subroutines<T: Band>(band: &mut T, top: &Operations, private: &Operations)
                                   -> Result<Subroutines> {

    let (_, mut offset) = get_double!(top, Private);
    offset += get_single!(private, Subrs);
    try!(band.jump(offset as u64));
    Value::read(band)
}

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{Charstrings, Names, Strings, Subroutines, TopDictionaries};
use self::compound::{Operator, Operations};
use self::primitive::Number;
