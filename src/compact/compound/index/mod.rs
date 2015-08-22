use std::io::Cursor;

use Result;
use band::{Band, ParametrizedValue, Value};
use compact::primitive::{Offset, OffsetSize};
use {compact, type2};

table_define! {
    pub Index {
        count   (u16        ),
        offSize (OffsetSize ),
        offset  (Vec<Offset>),
        data    (Vec<u8>    ),
    }
}

impl Index {
    pub fn get(&self, i: usize) -> Option<&[u8]> {
        if i >= self.count as usize {
            return None;
        }
        let from = self.offset[i] as usize - 1;
        let until = self.offset[i + 1] as usize - 1;
        Some(&self.data[from..until])
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
            let value = try!(ParametrizedValue::read(band, offSize));
            if i == 0 && value != 1 || i > 0 && value <= offset[i - 1] {
                raise!("found a malformed index");
            }
            offset.push(value);
        }
        let data = try!(ParametrizedValue::read(band, offset[count as usize] as usize - 1));
        Ok(Index { count: count, offSize: offSize, offset: offset, data: data })
    }
}

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
    pub CharstringIndex {
    }
}

index!(DictionaryIndex);
index!(NameIndex);
index!(SubroutineIndex);

impl CharstringIndex {
    pub fn read<T: Band>(band: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharstringIndex { index: try!(Value::read(band)) },
            _ => raise!("found char-string data with an unknown format"),
        })
    }

    pub fn get(&self, i: usize) -> Result<Option<type2::compound::Operations>> {
        let chunk = match self.index.get(i) {
            Some(chunk) => chunk,
            _ => return Ok(None),
        };
        let mut band = Cursor::new(chunk);
        Ok(Some(try!(Value::read(&mut band))))
    }
}

impl DictionaryIndex {
    pub fn get(&self, i: usize) -> Result<Option<compact::compound::Operations>> {
        let chunk = match self.index.get(i) {
            Some(chunk) => chunk,
            _ => return Ok(None),
        };
        let mut band = Cursor::new(chunk);
        Ok(Some(try!(Value::read(&mut band))))
    }
}

impl NameIndex {
    #[inline]
    pub fn get(&self, i: usize) -> Option<String> {
        self.index.get(i).and_then(|chunk| match chunk[0] {
            0 => None,
            _ => Some(String::from_utf8_lossy(chunk).into_owned()),
        })
    }
}

impl SubroutineIndex {
    pub fn get(&self, i: usize) -> Result<Option<type2::compound::Operations>> {
        let chunk = match self.index.get(i) {
            Some(chunk) => chunk,
            _ => return Ok(None),
        };
        let mut band = Cursor::new(chunk);
        Ok(Some(try!(Value::read(&mut band))))
    }
}

mod string;

pub use self::string::StringIndex;
