use std::io::Cursor;

use Result;
use band::{Band, ParametrizedValue, Value};
use compact::compound::Operations;
use compact::primitive::{Offset, OffsetSize};

table_define! {
    pub Index {
        count   (u16         ),
        offSize (OffsetSize  ),
        offset  (Vec<Offset> ),
        data    (Vec<Vec<u8>>),
    }
}

impl Value for Index {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let count = try!(band.take::<u16>());
        if count == 0 {
            return Ok(Index::default());
        }
        let offSize = try!(band.take::<OffsetSize>());
        let mut offset = Vec::with_capacity(count as usize + 1);
        for i in 0..(count as usize + 1) {
            let value = try!(Offset::read(band, offSize));
            if i == 0 && value != Offset(1) || i > 0 && value <= offset[i - 1] {
                raise!("found a malformed index");
            }
            offset.push(value);
        }
        let mut data = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            let size = (offset[i + 1].as_u32() - offset[i].as_u32()) as usize;
            data.push(try!(ParametrizedValue::read(band, size)));
        }
        Ok(Index { count: count, offSize: offSize, offset: offset, data: data })
    }
}

deref! { Index::data => [Vec<u8>] }

macro_rules! index {
    ($(#[$attribute:meta])* $structure:ident) => (
        index_define! { $(#[$attribute])* pub $structure {} }
        index_implement! { $structure }
    );
}

macro_rules! index_define {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident: $kind:ty,)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure {
            index: ::compact::compound::Index,
            $($field: $kind,)*
        }

        deref! { $structure::index => ::compact::compound::Index }
    );
}

macro_rules! index_implement {
    ($structure:ident) => (
        impl ::band::Value for $structure {
            #[inline]
            fn read<T: ::band::Band>(band: &mut T) -> ::Result<Self> {
                Ok($structure { index: try!(::band::Value::read(band)) })
            }
        }
    );
}

index_define! {
    pub Charstrings {
        format: i32,
    }
}

index!(TopDictionaries);
index!(Names);
index!(Subroutines);

impl ParametrizedValue<i32> for Charstrings {
    fn read<T: Band>(band: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => Charstrings { index: try!(Value::read(band)), format: format },
            _ => raise!("found an unknown charstring format"),
        })
    }
}

impl TopDictionaries {
    pub fn into_vec(self) -> Result<Vec<Operations>> {
        let TopDictionaries { index: Index { data, .. } } = self;
        let mut vector = Vec::with_capacity(data.len());
        for chunk in data {
            vector.push(try!(Value::read(&mut Cursor::new(chunk))));
        }
        Ok(vector)
    }
}

impl Names {
    pub fn into_vec(self) -> Result<Vec<String>> {
        let Names { index: Index { data, .. } } = self;
        let mut vector = Vec::with_capacity(data.len());
        for chunk in data {
            vector.push(match String::from_utf8(chunk) {
                Ok(string) => string,
                Err(chunk) => String::from_utf8_lossy(&chunk.into_bytes()).into_owned(),
            });
        }
        Ok(vector)
    }
}

mod string;

pub use self::string::Strings;
