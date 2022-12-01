use crate::{Result, Tape, Value};

macro_rules! reject(() => (raise!("found a malformed number")));

/// A number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    /// An integer number.
    Integer(i32),
    /// A real number.
    Real(f32),
}

impl From<f32> for Number {
    #[inline]
    fn from(value: f32) -> Self {
        Number::Real(value)
    }
}

impl From<i32> for Number {
    #[inline]
    fn from(value: i32) -> Self {
        Number::Integer(value)
    }
}

impl Value for Number {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let first = tape.take::<u8>()?;
        Ok(match first {
            0x20..=0xf6 => Number::Integer(first as i32 - 139),
            0xf7..=0xfa => {
                Number::Integer((first as i32 - 247) * 256 + tape.take::<u8>()? as i32 + 108)
            }
            0xfb..=0xfe => {
                Number::Integer(-(first as i32 - 251) * 256 - tape.take::<u8>()? as i32 - 108)
            }
            0x1c => Number::Integer(tape.take::<u16>()? as i16 as i32),
            0x1d => Number::Integer(tape.take::<u32>()? as i32),
            0x1e => Number::Real(parse(tape)?),
            _ => reject!(),
        })
    }
}

fn parse<T: Tape>(tape: &mut T) -> Result<f32> {
    let mut buffer = String::new();
    let mut byte = 0;
    let mut high = true;
    loop {
        let nibble = match high {
            true => {
                byte = tape.take::<u8>()?;
                byte >> 4
            }
            false => byte & 0x0f,
        };
        high = !high;
        match nibble {
            0..=9 => buffer.push(('0' as u8 + nibble) as char),
            0x0a => buffer.push('.'),
            0x0b => buffer.push('e'),
            0x0c => buffer.push_str("e-"),
            0x0e => buffer.push('-'),
            0x0f => break,
            _ => reject!(),
        }
    }
    match buffer.parse() {
        Ok(value) => Ok(value),
        _ => reject!(),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Number;
    use crate::Tape;

    #[test]
    fn integer() {
        macro_rules! read(
            ($tape:expr) => (
                match $tape.take().unwrap() {
                    Number::Integer(value) => value,
                    _ => unreachable!(),
                }
            )
        );

        let mut tape = Cursor::new(vec![0x8b]);
        assert_eq!(read!(tape), 0);

        let mut tape = Cursor::new(vec![0xef]);
        assert_eq!(read!(tape), 100);

        let mut tape = Cursor::new(vec![0x27]);
        assert_eq!(read!(tape), -100);

        let mut tape = Cursor::new(vec![0xfa, 0x7c]);
        assert_eq!(read!(tape), 1000);

        let mut tape = Cursor::new(vec![0xfe, 0x7c]);
        assert_eq!(read!(tape), -1000);

        let mut tape = Cursor::new(vec![0x1c, 0x27, 0x10]);
        assert_eq!(read!(tape), 10000);

        let mut tape = Cursor::new(vec![0x1c, 0xd8, 0xf0]);
        assert_eq!(read!(tape), -10000);

        let mut tape = Cursor::new(vec![0x1d, 0x00, 0x01, 0x86, 0xa0]);
        assert_eq!(read!(tape), 100000);

        let mut tape = Cursor::new(vec![0x1d, 0xff, 0xfe, 0x79, 0x60]);
        assert_eq!(read!(tape), -100000);
    }

    #[test]
    fn real() {
        macro_rules! read(
            ($tape:expr) => (
                match $tape.take().unwrap() {
                    Number::Real(value) => value,
                    _ => unreachable!(),
                }
            )
        );

        let mut tape = Cursor::new(vec![0x1e, 0xe2, 0xa2, 0x5f, 0x0f]);
        assert!((read!(tape) + 2.25).abs() < 1e-14);

        let mut tape = Cursor::new(vec![0x1e, 0x0a, 0x14, 0x05, 0x41, 0xc3, 0xff, 0x0f]);
        assert!((read!(tape) - 0.140541e-3).abs() < 1e-14);
    }
}
