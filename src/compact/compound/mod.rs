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
        impl ::tape::Value for $structure {
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(table.$field = try!(::tape::Value::read(tape));)+
                Ok(table)
            }
        }
    );
}

mod char_set;
mod encoding;
mod header;
mod index;
mod operation;

pub use self::char_set::{CharSet, CharSet1, CharSetRange1};
pub use self::encoding::Encoding;
pub use self::header::Header;
pub use self::index::{CharStrings, Index, Names};
pub use self::index::{Strings, Subroutines, TopDictionaries};
pub use self::operation::{Operation, Operations, Operator};
