use std::mem;

use Result;
use band::{Band, ParametrizedValue};
use compact::primitive::OffsetSize;

pub type Offset = u32;

#[cfg(target_endian = "big")]
macro_rules! assemble(
    ($hi:expr, $me:expr, $lo:expr) => ([0, $hi, $me, $lo]);
);

#[cfg(target_endian = "little")]
macro_rules! assemble(
    ($hi:expr, $me:expr, $lo:expr) => ([$lo, $me, $hi, 0]);
);

impl ParametrizedValue<OffsetSize> for Offset {
    fn read<T: Band>(band: &mut T, size: OffsetSize) -> Result<Self> {
        Ok(match size {
            1 => try!(band.take::<u8>()) as Offset,
            2 => try!(band.take::<u16>()) as Offset,
            3 => {
                let trio: [u8; 3] = read!(band, 3);
                unsafe { mem::transmute::<_, u32>(assemble!(trio[0], trio[1], trio[2])) }
            },
            4 => try!(band.take::<u32>()),
            _ => raise!("found an invalid size"),
        })
    }
}

#[cfg(test)]
mod tests {
    use band::ParametrizedValue;
    use compact::primitive::Offset;
    use std::io::Cursor;

    #[test]
    fn read() {
        let mut band = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        assert_eq!(Offset::read(&mut band, 1).unwrap(), 0x01);
        assert_eq!(Offset::read(&mut band, 2).unwrap(), 0x0203);
        assert_eq!(Offset::read(&mut band, 3).unwrap(), 0x040506);
        assert_eq!(Offset::read(&mut band, 4).unwrap(), 0x0708090a);
    }
}
