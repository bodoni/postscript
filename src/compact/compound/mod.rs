//! Compound data types.

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (
        table_define! { $(#[$attribute])* pub $structure { $($field ($kind),)+ } }
        table_implement! { pub $structure { $($field,)+ } }
    );
}

macro_rules! table_define {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
}

macro_rules! table_implement {
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

macro_rules! deref {
    ($structure:ident($field:tt) => $target:ty) => (itemize! {
        impl ::std::ops::Deref for $structure {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
    });
}

macro_rules! itemize(($code:item) => ($code));

mod char_set;
mod encoding;
mod header;
mod index;
mod operation;

pub use self::char_set::{CharSet, CharSet1, CharSetRange1};
pub use self::encoding::Encoding;
pub use self::header::Header;
pub use self::index::Index;
pub use self::index::{CharStringIndex, DictionaryIndex, NameIndex, StringIndex, SubroutineIndex};
pub use self::operation::{Operator, Operand, Operations};
