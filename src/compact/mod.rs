//! Compact font format.

#![allow(non_snake_case)]

use std::io::{Read, Seek};

use Result;
use band::Value;

pub struct Compact {
    pub header: Header,
}

impl Compact {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Self> {
        Ok(Compact { header: try!(Value::read(reader)) })
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

mod header;

pub use self::header::Header;
