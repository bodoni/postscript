//! The compact font format.

use std::io::{Cursor, Read, Seek};

use Result;
use tape::{Tape, Value, Walue};

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
    /// Parse a font set.
    #[inline]
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Self> {
        Value::read(reader)
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

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let start = try!(tape.position());

        let header = try!(Header::read(tape));
        try!(tape.jump(start + header.header_size as u64));
        let names = try!(try!(Names::read(tape)).into_vec());
        let top_dictionaries = try!(try!(TopDictionaries::read(tape)).into_vec());
        let strings = try!(Strings::read(tape));
        let global_subroutines = try!(Subroutines::read(tape));

        let mut encodings = vec![];
        let mut charsets = vec![];
        let mut charstrings = vec![];
        let mut private_dictionaries = vec![];
        let mut local_subroutines = vec![];
        for (i, top) in top_dictionaries.iter().enumerate() {
            encodings.push(match get_single!(top, Encoding) {
                0 => Encoding::Standard,
                1 => Encoding::Expert,
                _ => unimplemented!(),
            });

            charstrings.push({
                try!(tape.jump(start + get_single!(top, Charstrings) as u64));
                try!(Charstrings::read(tape, get_single!(top, CharstringType)))
            });

            charsets.push(match get_single!(top, Charset) {
                0 => Charset::ISOAdobe,
                1 => Charset::Expert,
                2 => Charset::ExpertSubset,
                offset => {
                    try!(tape.jump(start + offset as u64));
                    try!(Charset::read(tape, charstrings[i].len()))
                },
            });

            private_dictionaries.push({
                let (size, offset) = get_double!(top, Private);
                try!(tape.jump(start + offset as u64));
                let chunk: Vec<u8> = try!(Walue::read(tape, size as usize));
                try!(Operations::read(&mut Cursor::new(chunk)))
            });

            local_subroutines.push({
                let (_, mut offset) = get_double!(top, Private);
                offset += get_single!(&private_dictionaries[i], Subrs);
                try!(tape.jump(start + offset as u64));
                try!(Subroutines::read(tape))
            });
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

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{Charstrings, Names, Strings, Subroutines, TopDictionaries};
use self::compound::{Operator, Operations};
use self::primitive::Number;
