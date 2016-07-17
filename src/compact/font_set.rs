use std::io::Cursor;

use {Result, Tape, Value};
use compact::{
    CharSet,
    Encoding,
    Header,
};
use compact::index::{
    CharStrings,
    Dictionaries,
    Names,
    Strings,
    Subroutines,
};
use compact::operation::{
    Operations,
    Operator,
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

macro_rules! get_single(
    ($operations:expr, $operator:ident) => ({
        match $operations.get_single(Operator::$operator) {
            Some(value) if is_i32!(value) => value as i32,
            _ => raise!("failed to process an operation ({})", stringify!($operator)),
        }
    });
);

macro_rules! get_double(
    ($operations:expr, $operator:ident) => (
        match $operations.get_double(Operator::$operator) {
            Some((value0, value1)) if is_i32!(value0) && is_i32!(value1) => {
                (value0 as i32, value1 as i32)
            },
            _ => raise!("failed to process an operation ({})", stringify!($operator)),
        }
    );
);

macro_rules! is_i32(
    ($value:ident) => ($value as i32 as f32 == $value);
);

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let start = try!(tape.position());

        let header = read_value!(tape, Header);
        try!(tape.jump(start + header.header_size as u64));
        let names = try!(read_value!(tape, Names).into());
        let global_dictionaries = try!(read_value!(tape, Dictionaries).into());
        let strings = read_value!(tape, Strings);
        let global_subroutines = read_value!(tape, Subroutines);

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
                read_walue!(tape, get_single!(dictionary, CharStringType), CharStrings)
            });

            char_sets.push(match get_single!(dictionary, CharSet) {
                0 => CharSet::ISOAdobe,
                1 => CharSet::Expert,
                2 => CharSet::ExpertSubset,
                offset => {
                    try!(tape.jump(start + offset as u64));
                    read_walue!(tape, char_strings[i].len())
                },
            });

            local_dictionaries.push({
                let (size, offset) = get_double!(dictionary, Private);
                try!(tape.jump(start + offset as u64));
                let chunk: Vec<u8> = read_walue!(tape, size as usize);
                read_value!(&mut Cursor::new(chunk), Operations)
            });

            local_subroutines.push({
                let (_, mut offset) = get_double!(dictionary, Private);
                offset += get_single!(&local_dictionaries[i], Subrs);
                try!(tape.jump(start + offset as u64));
                read_value!(tape)
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
