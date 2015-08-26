//! Compound data types.

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (
        table_define! { $(#[$attribute])* pub $structure { $($field ($kind),)+ } }
        table_read! { pub $structure { $($field,)+ } }
    );
}

macro_rules! table_define {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
}

macro_rules! table_read {
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

mod charset;
mod encoding;
mod header;
mod index;
mod operation;

pub use self::charset::{Charset, Charset1, CharsetRange1};
pub use self::encoding::Encoding;
pub use self::header::Header;
pub use self::index::{Charstrings, Index, Names};
pub use self::index::{Strings, Subroutines, TopDictionaries};
pub use self::operation::{Operation, Operations, Operator};
