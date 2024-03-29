use crate::Result;

/// An offset.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Offset(pub u32);

/// An offset size.
pub type OffsetSize = u8;

impl crate::walue::Read<'static> for Offset {
    type Parameter = OffsetSize;

    fn read<T: crate::tape::Read>(tape: &mut T, size: OffsetSize) -> Result<Self> {
        Ok(Offset(match size {
            1 => tape.take::<u8>()? as u32,
            2 => tape.take::<u16>()? as u32,
            3 => {
                let value: [u8; 3] = tape.take()?;
                u32::from_be_bytes([0, value[0], value[1], value[2]])
            }
            4 => tape.take::<u32>()?,
            _ => raise!("found a malformed offset"),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::compact1::Offset;
    use crate::walue::Read;

    #[test]
    fn read() {
        let mut tape = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        assert_eq!(Offset::read(&mut tape, 1).unwrap().0, 0x01);
        assert_eq!(Offset::read(&mut tape, 2).unwrap().0, 0x0203);
        assert_eq!(Offset::read(&mut tape, 3).unwrap().0, 0x040506);
        assert_eq!(Offset::read(&mut tape, 4).unwrap().0, 0x0708090a);
    }
}
