use std::io::Cursor;

use Result;
use band::{Band, ParametrizedValue, Value};
use compact::compound::Operations;
use compact::primitive::{Offset, OffsetSize};

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

index!(CharStringIndex);
index!(DictionaryIndex);
index!(NameIndex);
index!(SubroutineIndex);

impl DictionaryIndex {
    pub fn get(&self, i: usize) -> Result<Option<Operations>> {
        let chunk = match self.0.get(i) {
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
        self.0.get(i).and_then(|chunk| match chunk[0] {
            0 => None,
            _ => Some(String::from_utf8_lossy(chunk).into_owned()),
        })
    }
}
