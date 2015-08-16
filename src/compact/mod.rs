//! Compact font format.

#![allow(non_snake_case)]

use std::io::{Read, Seek};

use Result;
use band::{Band, Value};

/// A font set.
pub struct FontSet {
    pub header: Header,
    pub name_index: NameIndex,
    pub top_dictionary: DictionaryIndex,
    pub string_index: StringIndex,
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
        let name_index = try!(NameIndex::read(band));
        let top_dictionary = try!(DictionaryIndex::read(band));
        if name_index.count != top_dictionary.count {
            raise!("the name and top dictionary indices do not match");
        }
        let string_index = try!(StringIndex::read(band));
        Ok(FontSet {
            header: header,
            name_index: name_index,
            top_dictionary: top_dictionary,
            string_index: string_index,
        })
    }
}

pub mod compound;
pub mod primitive;

use self::compound::{DictionaryIndex, Header, NameIndex, StringIndex};
