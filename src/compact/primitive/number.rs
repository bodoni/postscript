use Result;
use band::{Band, Value};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Number {
    Integer(i32),
    Real(f32),
}

use self::Number::*;

impl Number {
    #[inline(always)]
    pub fn as_i32(&self) -> i32 {
        match self {
            &Integer(value) => value,
            &Real(value) => value as i32,
        }
    }

    #[inline(always)]
    pub fn as_f32(&self) -> f32 {
        match self {
            &Integer(value) => value as f32,
            &Real(value) => value,
        }
    }
}

impl Value for Number {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        macro_rules! read(($kind:ident) => (try!(band.take::<$kind>())));
        let first = read!(u8);
        Ok(match first {
            0x20...0xf6 => Integer(first as i32 - 139),
            0xf7...0xfa => Integer((first as i32 - 247) * 256 + read!(u8) as i32 + 108),
            0xfb...0xfe => Integer(-(first as i32 - 251) * 256 - read!(u8) as i32 - 108),
            0x1c => Integer(read!(u16) as i16 as i32),
            0x1d => Integer(read!(u32) as i32),
            0x1e => Real(try!(read_real(band))),
            _ => raise!("found a malformed number"),
        })
    }
}

fn read_real<T: Band>(band: &mut T) -> Result<f32> {
    let mut buffer = String::new();
    let mut byte = 0;
    let mut high = true;
    loop {
        let nibble = match high {
            true => {
                byte = try!(band.take::<u8>());
                byte >> 4
            },
            false => byte & 0x0f,
        };
        high = !high;
        match nibble {
            0...9 => buffer.push(('0' as u8 + nibble) as char),
            0x0a => buffer.push('.'),
            0x0b => buffer.push('e'),
            0x0c => buffer.push_str("e-"),
            0x0e => buffer.push('-'),
            0x0f => break,
            _ => raise!("found a malformed real number"),
        }
    }
    match buffer.parse() {
        Ok(value) => Ok(value),
        _ => raise!("failed to parse a real number"),
    }
}

#[cfg(test)]
mod tests {
    use band::Value;
    use compact::primitive::Number;
    use std::io::Cursor;

    #[test]
    fn integer() {
        macro_rules! read(($band:expr) => (Number::read(&mut $band).unwrap().as_i32()));

        let mut band = Cursor::new(vec![0x8b]);
        assert_eq!(read!(band), 0);

        let mut band = Cursor::new(vec![0xef]);
        assert_eq!(read!(band), 100);

        let mut band = Cursor::new(vec![0x27]);
        assert_eq!(read!(band), -100);

        let mut band = Cursor::new(vec![0xfa, 0x7c]);
        assert_eq!(read!(band), 1000);

        let mut band = Cursor::new(vec![0xfe, 0x7c]);
        assert_eq!(read!(band), -1000);

        let mut band = Cursor::new(vec![0x1c, 0x27, 0x10]);
        assert_eq!(read!(band), 10000);

        let mut band = Cursor::new(vec![0x1c, 0xd8, 0xf0]);
        assert_eq!(read!(band), -10000);

        let mut band = Cursor::new(vec![0x1d, 0x00, 0x01, 0x86, 0xa0]);
        assert_eq!(read!(band), 100000);

        let mut band = Cursor::new(vec![0x1d, 0xff, 0xfe, 0x79, 0x60]);
        assert_eq!(read!(band), -100000);
    }

    #[test]
    fn real() {
        macro_rules! read(($band:expr) => (Number::read(&mut $band).unwrap().as_f32()));

        let mut band = Cursor::new(vec![0x1e, 0xe2, 0xa2, 0x5f, 0x0f]);
        assert_eq!(read!(band), -2.25);

        let mut band = Cursor::new(vec![0x1e, 0x0a, 0x14, 0x05, 0x41, 0xc3, 0xff, 0x0f]);
        assert!((read!(band) - 0.140541e-3).abs() < 1e-14);
    }
}
