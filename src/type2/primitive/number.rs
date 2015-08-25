use std::ops::{Neg, Not};

use Result;
use band::{Band, Value};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Number {
    Integer(i32),
    Real(f32),
}

use self::Number::*;

impl Number {
    #[inline]
    pub fn abs(self) -> Number {
        match self {
            Integer(value) => Integer(value.abs()),
            Real(value) => Real(value.abs()),
        }
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        match *self {
            Integer(value) => value,
            Real(value) => value as i32,
        }
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        match *self {
            Integer(value) => value as f32,
            Real(value) => value,
        }
    }

    #[inline]
    pub fn sqrt(self) -> Number {
        match self {
            Integer(value) => Integer((value as f32).sqrt() as i32),
            Real(value) => Real(value.sqrt()),
        }
    }
}

impl Value for Number {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        const FIXED_SCALING: f32 = 1f32 / (1 << 16) as f32;
        macro_rules! read(($kind:ident) => (try!(band.take::<$kind>())));
        let first = read!(u8);
        Ok(match first {
            0x20...0xf6 => Integer(first as i32 - 139),
            0xf7...0xfa => Integer((first as i32 - 247) * 256 + read!(u8) as i32 + 108),
            0xfb...0xfe => Integer(-(first as i32 - 251) * 256 - read!(u8) as i32 - 108),
            0x1c => Integer(read!(u16) as i16 as i32),
            0xff => Real(FIXED_SCALING * (read!(u32) as f32)),
            _ => raise!("found a malformed number"),
        })
    }
}

impl Neg for Number {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Integer(value) => Integer(-value),
            Real(value) => Real(-value),
        }
    }
}

impl Not for Number {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Integer(value) => Integer(if value != 0 { 1 } else { 0 }),
            Real(value) => Integer(if value != 0.0 { 1 } else { 0 }),
        }
    }
}

#[cfg(test)]
mod tests {
    use band::Value;
    use std::io::Cursor;
    use type2::primitive::Number;

    #[test]
    fn real() {
        macro_rules! read(($band:expr) => (Number::read(&mut $band).unwrap().as_f32()));

        let mut band = Cursor::new(vec![0xff, 0x00, 0x01, 0x04, 0x5a]);
        assert_eq!(format!("{:.3}", read!(band)), "1.017");
    }
}
