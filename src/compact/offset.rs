use std::mem;

use {Result, Tape, Walue};

/// An offset.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Offset(pub u32);

/// An offset size.
pub type OffsetSize = u8;

#[cfg(target_endian = "big")]
macro_rules! assemble(
    ($hi:expr, $me:expr, $lo:expr) => ([0, $hi, $me, $lo]);
);

#[cfg(target_endian = "little")]
macro_rules! assemble(
    ($hi:expr, $me:expr, $lo:expr) => ([$lo, $me, $hi, 0]);
);

impl From<Offset> for u32 {
    #[inline(always)]
    fn from(offset: Offset) -> u32 {
        offset.0
    }
}

impl Walue<OffsetSize> for Offset {
    fn read<T: Tape>(tape: &mut T, size: OffsetSize) -> Result<Self> {
        Ok(Offset(match size {
            1 => try!(tape.take::<u8>()) as u32,
            2 => try!(tape.take::<u16>()) as u32,
            3 => {
                let trio: [u8; 3] = read_value!(tape);
                unsafe { mem::transmute::<_, u32>(assemble!(trio[0], trio[1], trio[2])) }
            },
            4 => try!(tape.take::<u32>()),
            _ => raise!("found an invalid size"),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use Walue;
    use compact::Offset;

    #[test]
    fn read() {
        let mut tape = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        assert_eq!(u32::from(Offset::read(&mut tape, 1).unwrap()), 0x01);
        assert_eq!(u32::from(Offset::read(&mut tape, 2).unwrap()), 0x0203);
        assert_eq!(u32::from(Offset::read(&mut tape, 3).unwrap()), 0x040506);
        assert_eq!(u32::from(Offset::read(&mut tape, 4).unwrap()), 0x0708090a);
    }
}
