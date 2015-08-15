//! Compact font format.

#![allow(non_snake_case)]

use std::io::{Read, Seek};

use Result;
use band::{Band, Value};

pub struct Compact {
    pub header: Header,
    pub name_index: NameIndex,
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
        Ok(Compact { header: header, name_index: name_index })
    }
}

macro_rules! spec {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (
        declare! { $(#[$attribute])* pub $structure { $($field ($kind),)+ } }
        implement! { pub $structure { $($field,)+ } }
    );
}

macro_rules! declare {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
}

macro_rules! implement {
    (pub $structure:ident { $($field:ident,)+ }) => (
        impl ::band::Value for $structure {
            fn read<T: ::band::Band>(band: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(table.$field = try!(::band::Value::read(band));)+
                Ok(table)
            }
        }
    );
}

macro_rules! itemize(
    ($code:item) => ($code);
);

pub mod primitive;

mod header;
mod index;

pub use self::header::Header;
pub use self::index::{Index, NameIndex};
