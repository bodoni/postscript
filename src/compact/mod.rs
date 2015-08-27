//! The compact font format.

use std::io::{Cursor, Read, Seek};

use Result;
use tape::{Tape, Value, ValueExt};

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

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let header = try!(Header::read(tape));
        try!(tape.jump(header.header_size as u64));
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
            encodings.push(try!(read_encoding(tape, top)));
            charstrings.push(try!(read_charstrings(tape, top)));

            let glyphs = charstrings[i].len();
            charsets.push(try!(read_charset(tape, top, glyphs)));

            private_dictionaries.push(try!(read_private_dictionary(tape, top)));

            let private = &private_dictionaries[i];
            local_subroutines.push(try!(read_local_subroutines(tape, top, private)));
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

fn read_encoding<T: Tape>(_: &mut T, top: &Operations) -> Result<Encoding> {
    Ok(match get_single!(top, Encoding) {
        0 => Encoding::Standard,
        1 => Encoding::Expert,
        _ => unimplemented!(),
    })
}

fn read_charset<T: Tape>(tape: &mut T, top: &Operations, glyphs: usize) -> Result<Charset> {
    match get_single!(top, Charset) {
        0 => Ok(Charset::ISOAdobe),
        1 => Ok(Charset::Expert),
        2 => Ok(Charset::ExpertSubset),
        offset => {
            try!(tape.jump(offset as u64));
            ValueExt::read(tape, glyphs)
        },
    }
}

fn read_charstrings<T: Tape>(tape: &mut T, top: &Operations) -> Result<Charstrings> {
    try!(tape.jump(get_single!(top, Charstrings) as u64));
    ValueExt::read(tape, get_single!(top, CharstringType))
}

fn read_private_dictionary<T: Tape>(tape: &mut T, top: &Operations) -> Result<Operations> {
    let (size, offset) = get_double!(top, Private);
    try!(tape.jump(offset as u64));
    let chunk: Vec<u8> = try!(ValueExt::read(tape, size as usize));
    Value::read(&mut Cursor::new(chunk))
}

fn read_local_subroutines<T: Tape>(tape: &mut T, top: &Operations, private: &Operations)
                                   -> Result<Subroutines> {

    let (_, mut offset) = get_double!(top, Private);
    offset += get_single!(private, Subrs);
    try!(tape.jump(offset as u64));
    Value::read(tape)
}

pub mod compound;
pub mod primitive;

use self::compound::{Charset, Encoding, Header};
use self::compound::{Charstrings, Names, Strings, Subroutines, TopDictionaries};
use self::compound::{Operator, Operations};
use self::primitive::Number;
