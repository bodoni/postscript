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

impl Value for Index {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let count = try!(Card16::read(band)) as usize;
        if count == 0 {
            return Ok(Index::default());
        }
        let offSize = try!(OffSize::read(band)) as usize;
        let mut offset = Vec::with_capacity(count + 1);
        for i in 0..(count + 1) {
            let value = try!(Walue::read(band, offSize));
            if i == 0 && value != 1 || i > 0 && value <= offset[i - 1] {
                raise!("found a malformed index");
            }
            offset.push(value);
        }
        let data = try!(Walue::read(band, offset[count] as usize - 1));
        Ok(Index {
            count: count as Card16,
            offSize: offSize as OffSize,
            offset: offset,
            data: data,
        })
    }
}
