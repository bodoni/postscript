use Result;
use band::{Band, Value, Walue};
use compact::primitive::*;

declare! {
    pub Index {
        count   (Card16     ),
        offSize (OffSize    ),
        offset  (Vec<Offset>),
        data    (Vec<Card8> ),
    }
}

pub type NameIndex = Index;

impl Index {
    pub fn strings(&self) -> Vec<String> {
        let count = self.count as usize;
        let mut strings = Vec::with_capacity(count);
        let mut from = 0;
        for i in 0..count {
            let until = self.offset[i + 1] as usize;
            let mut slice = &self.data[from..until];
            if let Some(j) = slice.iter().position(|&byte| byte == 0) {
                slice = &slice[0..j];
            }
            if slice.is_empty() {
                continue;
            }
            let string = String::from_utf8_lossy(slice);
            strings.push(string.into_owned());
            from = until;
        }
        strings
    }
}

impl Value for Index {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let count = try!(Card16::read(band)) as usize;
        if count == 0 {
            return Ok(Index::default());
        }
        let offSize = try!(OffSize::read(band)) as usize;
        let mut offset = Vec::with_capacity(count + 1);
        for _ in 0..(count + 1) {
            offset.push(try!(Walue::read(band, offSize)));
        }
        let data = try!(Walue::read(band, offset[count] as usize));
        Ok(Index {
            count: count as Card16,
            offSize: offSize as OffSize,
            offset: offset,
            data: data,
        })
    }
}
