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
    pub charsets: Vec<Charset>,
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

        let (mut encodings, mut charsets) = (vec![], vec![]);
        for i in 0..(dictionaries.count as usize) {
            let dictionary = match try!(dictionaries.get(i)) {
                Some(dictionary) => dictionary,
                _ => raise!("failed to find a dictionary"),
            };
            encodings.push(try!(read_encoding(band, &dictionary)));
            charsets.push(try!(read_charset(band, &dictionary)));
        }

        Ok(FontSet {
            header: header,
            names: names,
            dictionaries: dictionaries,
            strings: strings,
            subroutines: subroutines,
            encodings: encodings,
            charsets: charsets,
        })
    }
}

fn read_encoding<T: Band>(_: &mut T, dictionary: &Operations) -> Result<Encoding> {
    let offset = match dictionary.get(Operator::Encoding) {
        Some(operands) => match (operands.len(), operands.get(0)) {
            (1, Some(&Operand::Integer(offset))) => offset,
            _ => raise!("found an encoding operator with invalid operands"),
        },
        _ => raise!("failed to identify an encoding"),
    };
    Ok(match offset {
        0 => Encoding::Standard,
        1 => Encoding::Expert,
        _ => unimplemented!(),
    })
}

fn read_charset<T: Band>(band: &mut T, dictionary: &Operations) -> Result<Charset> {
    let offset = match dictionary.get(Operator::charset) {
        Some(operands) => match (operands.len(), operands.get(0)) {
            (1, Some(&Operand::Integer(offset))) => offset,
            _ => raise!("found a charset operator with invalid operands"),
        },
        _ => raise!("failed to identify a charset"),
    };
    match offset {
        0 => Ok(Charset::ISOAdobe),
        1 => Ok(Charset::Expert),
        2 => Ok(Charset::ExpertSubset),
        _ => {
            try!(band.jump(offset as u64));
            Value::read(band)
        }
    }
}

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
use self::compound::{Operator, Operand, Operations};
