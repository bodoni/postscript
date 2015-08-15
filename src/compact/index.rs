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
