use Result;
use band::{Band, ParametrizedValue, Value};
use compact::primitive::*;

declare! {
    pub Index {
        count   (Card16     ),
        offSize (OffSize    ),
        offset  (Vec<Offset>),
        data    (Vec<Card8> ),
    }
}

impl Index {
    pub fn get(&self, i: usize) -> Option<&[Card8]> {
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
        let count = try!(Card16::read(band));
        if count == 0 {
            return Ok(Index::default());
        }
        let offSize = try!(OffSize::read(band));
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
