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

macro_rules! index {
    ($(#[$attribute:meta])* pub $structure:ident) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure(pub ::compact::compound::Index);

        impl ::band::Value for $structure {
            #[inline]
            fn read<T: ::band::Band>(band: &mut T) -> ::Result<Self> {
                Ok($structure(try!(::band::Value::read(band))))
            }
        }

        impl ::std::ops::Deref for $structure {
            type Target = ::compact::compound::Index;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    );
}

macro_rules! itemize(($code:item) => ($code));

macro_rules! lookup {
    ($what:ident, $from:ty => $into:ty { $($key:expr => $value:expr,)+ }) => ({
        use std::collections::HashMap;
        use std::sync::{ONCE_INIT, Once};

        unsafe {
            static mut MAP: *const HashMap<$from, $into> = 0 as *const _;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                let mut map: HashMap<$from, $into> = HashMap::new();
                $(map.insert($key, $value);)+
                MAP = ::std::mem::transmute(Box::new(map));
            });
            (&*MAP).get(&$what).cloned()
        }
    });
}

mod dictionary_index;
mod encoding;
mod header;
mod index;
mod name_index;
mod operation;
mod string_index;
mod subroutine_index;

pub use self::dictionary_index::DictionaryIndex;
pub use self::encoding::Encoding;
pub use self::header::Header;
pub use self::index::Index;
pub use self::name_index::NameIndex;
pub use self::operation::{Operand, Operator};
pub use self::string_index::StringIndex;
pub use self::subroutine_index::SubroutineIndex;
