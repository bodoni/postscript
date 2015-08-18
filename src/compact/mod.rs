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
        let encodings = try!(read_encodings(&dictionaries));
        Ok(FontSet {
            header: header,
            names: names,
            dictionaries: dictionaries,
            strings: strings,
            subroutines: subroutines,
            encodings: encodings,
        })
    }
}

fn read_encodings(dictionaries: &DictionaryIndex) -> Result<Vec<Encoding>> {
    let mut encodings = vec![];
    for i in 0..(dictionaries.count as usize) {
        let mut dictionary = match try!(dictionaries.get(i)) {
            Some(dictionary) => dictionary,
            _ => raise!("failed to find a dictionary"),
        };
        let offset = match dictionary.remove(&Operator::Encoding)
                                     .or_else(|| Operator::Encoding.default()) {
            Some(ref operands) => match (operands.len(), operands.get(0)) {
                (1, Some(&Operand::Integer(offset))) => offset,
                _ => raise!("found an encoding operator with invalid operands"),
            },
            _ => raise!("failed to identify an encoding"),
        };
        encodings.push(match offset {
            0 => Encoding::Standard,
            1 => Encoding::Expert,
            _ => raise!("found an unsupported encoding"),
        });
    }
    Ok(encodings)
}

pub mod compound;
pub mod primitive;

use self::compound::{DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
use self::compound::{Encoding, Header, Operand, Operator};
