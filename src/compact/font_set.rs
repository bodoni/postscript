use std::io::{Cursor, Read, Seek};

use {Result, Tape, Value, Walue};
use compact::{
    CharSet,
    CharStrings,
    Dictionaries,
    Encoding,
    Header,
    Names,
    Number,
    Operations,
    Operator,
    Strings,
    Subroutines,
};

/// A font set.
#[derive(Clone, Debug)]
pub struct FontSet {
    pub header: Header,
    pub names: Vec<String>,
    pub strings: Strings,
    pub encodings: Vec<Encoding>,
    pub char_sets: Vec<CharSet>,
    pub char_strings: Vec<CharStrings>,
    pub global_dictionaries: Vec<Operations>,
    pub global_subroutines: Subroutines,
    pub local_dictionaries: Vec<Operations>,
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
        let names = try!(try!(Names::read(tape)).into());
        let global_dictionaries = try!(try!(Dictionaries::read(tape)).into());
        let strings = try!(Strings::read(tape));
        let global_subroutines = try!(Subroutines::read(tape));

        let mut encodings = vec![];
        let mut char_sets = vec![];
        let mut char_strings = vec![];
        let mut local_dictionaries = vec![];
        let mut local_subroutines = vec![];
        for (i, dictionary) in global_dictionaries.iter().enumerate() {
            encodings.push(match get_single!(dictionary, Encoding) {
                0 => Encoding::Standard,
                1 => Encoding::Expert,
                _ => unimplemented!(),
            });

            char_strings.push({
                try!(tape.jump(start + get_single!(dictionary, CharStrings) as u64));
                try!(CharStrings::read(tape, get_single!(dictionary, CharStringType)))
            });

            char_sets.push(match get_single!(dictionary, CharSet) {
                0 => CharSet::ISOAdobe,
                1 => CharSet::Expert,
                2 => CharSet::ExpertSubset,
                offset => {
                    try!(tape.jump(start + offset as u64));
                    try!(CharSet::read(tape, char_strings[i].len()))
                },
            });

            local_dictionaries.push({
                let (size, offset) = get_double!(dictionary, Private);
                try!(tape.jump(start + offset as u64));
                let chunk: Vec<u8> = try!(Walue::read(tape, size as usize));
                try!(Operations::read(&mut Cursor::new(chunk)))
            });

            local_subroutines.push({
                let (_, mut offset) = get_double!(dictionary, Private);
                offset += get_single!(&local_dictionaries[i], Subrs);
                try!(tape.jump(start + offset as u64));
                try!(Subroutines::read(tape))
            });
        }

        Ok(FontSet {
            header: header,
            names: names,
            strings: strings,
            encodings: encodings,
            char_sets: char_sets,
            char_strings: char_strings,
            global_dictionaries: global_dictionaries,
            global_subroutines: global_subroutines,
            local_dictionaries: local_dictionaries,
            local_subroutines: local_subroutines,
        })
    }
}
