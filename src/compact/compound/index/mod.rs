use std::io::Cursor;

use Result;
use band::{Band, ParametrizedValue, Value};
use compact::primitive::{Offset, OffsetSize};
use {compact, type2};

pub type Index = Vec<Vec<u8>>;

impl Value for Index {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let count = try!(band.take::<u16>()) as usize;
        if count == 0 {
            return Ok(vec![]);
        }
        let offset_size = try!(band.take::<OffsetSize>());
        let mut offsets = Vec::with_capacity(count + 1);
        for i in 0..(count + 1) {
            let offset = try!(Offset::read(band, offset_size)).as_u32() as usize;
            if i == 0 && offset != 1 || i > 0 && offset <= offsets[i - 1] {
                raise!("found a malformed index");
            }
            offsets.push(offset);
        }
        let mut chunks = Vec::with_capacity(count);
        for i in 0..count {
            chunks.push(try!(ParametrizedValue::read(band, offsets[i + 1] - offsets[i])));
        }
        Ok(chunks)
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
        format: i32,
    }
}

index!(TopDictionaryIndex);
index!(NameIndex);
index!(SubroutineIndex);

impl CharstringIndex {
    pub fn get(&self, i: usize) -> Result<Option<type2::compound::Operations>> {
        if i >= self.len() {
            return Ok(None);
        }
        Ok(Some(try!(Value::read(&mut Cursor::new(&*self.index[i])))))
    }
}

impl ParametrizedValue<i32> for CharstringIndex {
    fn read<T: Band>(band: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharstringIndex { index: try!(Value::read(band)), format: format },
            _ => raise!("found an unknown charstring format"),
        })
    }
}

impl TopDictionaryIndex {
    pub fn get(&self, i: usize) -> Result<Option<compact::compound::Operations>> {
        if i >= self.len() {
            return Ok(None);
        }
        Ok(Some(try!(Value::read(&mut Cursor::new(&*self.index[i])))))
    }
}

impl NameIndex {
    pub fn into_vec(self) -> Vec<String> {
        let NameIndex { index } = self;
        let mut vector = Vec::with_capacity(index.len());
        for chunk in index {
            vector.push(match String::from_utf8(chunk) {
                Ok(string) => string,
                Err(chunk) => String::from_utf8_lossy(&chunk.into_bytes()).into_owned(),
            });
        }
        vector
    }
}

impl SubroutineIndex {
    pub fn get(&self, i: usize) -> Result<Option<type2::compound::Operations>> {
        if i >= self.len() {
            return Ok(None);
        }
        Ok(Some(try!(Value::read(&mut Cursor::new(&*self.index[i])))))
    }

    pub fn get_biased(&self, mut i: isize, format: i32)
                      -> Result<Option<type2::compound::Operations>> {

        let bias = match (format, self.len()) {
            (1, _) => 0,
            (2, count) if count < 1240 => 107,
            (2, count) if count < 33900 => 1131,
            (2, _) => 32768,
            _ => raise!("found an unknown bias format"),
        };
        i = i + bias;
        assert!(i >= 0);
        self.get(i as usize)
    }
}

mod string;

pub use self::string::StringIndex;
