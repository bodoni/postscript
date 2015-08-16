use Result;
use band::{Band, ParametrizedValue, Value};
use compact::primitive::{Offset, OffsetSize};

declare! {
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
