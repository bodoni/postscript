//! Compact font format.

#![allow(non_snake_case)]

use std::io::{Read, Seek};

use Result;
use band::{Band, Value};

/// A font set encoded in the compact font format.
pub struct Compact {
    pub header: Header,
    pub name_index: NameIndex,
    pub dictionary_index: DictionaryIndex,
    pub string_index: StringIndex,
}

impl Compact {
    #[inline]
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Self> {
        Value::read(reader)
    }
}

impl Value for Compact {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let header = try!(Header::read(band));
        try!(band.jump(header.hdrSize as u64));
        let name_index = try!(NameIndex::read(band));
        let dictionary_index = try!(DictionaryIndex::read(band));
        if name_index.count != dictionary_index.count {
            raise!("the name and top dictionary indices do not match");
        }
        let string_index = try!(StringIndex::read(band));
        Ok(Compact {
            header: header,
            name_index: name_index,
            dictionary_index: dictionary_index,
            string_index: string_index,
        })
    }
}

pub mod compound;
pub mod primitive;

use self::compound::{DictionaryIndex, Header, NameIndex, StringIndex};
