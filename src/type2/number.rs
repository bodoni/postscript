use std::ops::Not;

use {Result, Tape, Value};

number!(Number);
use self::Number::*;

#[doc(hidden)]
impl Number {
    #[inline]
    pub fn abs(self) -> Self {
        match self {
            Integer(value) => Integer(value.abs()),
            Real(value) => Real(value.abs()),
        }
    }

    #[inline]
    pub fn and(self, that: Self) -> Self {
        (!bool::from(self) && !bool::from(that)).into()
    }

    #[inline]
    pub fn equal(self, that: Self) -> Self {
        match (self, that) {
            (Integer(this), Integer(that)) => this == that,
            (Real(this), Real(that)) => this == that,
            (Integer(this), Real(that)) => this as f32 == that,
            (Real(this), Integer(that)) => this == that as f32,
        }.into()
    }

    #[inline]
    pub fn or(self, that: Self) -> Self {
        (!bool::from(self) || !bool::from(that)).into()
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        match self {
            Integer(value) => Integer((value as f32).sqrt() as i32),
            Real(value) => Real(value.sqrt()),
        }
    }
}

#[doc(hidden)]
impl Not for Number {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        (!bool::from(self)).into()
    }
}

impl Value for Number {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        const FIXED_SCALING: f32 = 1f32 / (1 << 16) as f32;
        macro_rules! read(($kind:ident) => (try!(tape.take::<$kind>())));
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

#[doc(hidden)]
impl From<Number> for bool {
    #[inline(always)]
    fn from(number: Number) -> bool {
        match number {
            Integer(value) => value == 0,
            Real(value) => value == 0.0,
        }
    }
}

#[doc(hidden)]
impl From<bool> for Number {
    #[inline(always)]
    fn from(yes: bool) -> Self {
        if yes { Integer(1) } else { Integer(0) }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use Value;
    use type2::Number;

    #[test]
    fn real() {
        macro_rules! read(($tape:expr) => (f32::from(Number::read(&mut $tape).unwrap())));

        let mut tape = Cursor::new(vec![0xff, 0x00, 0x01, 0x04, 0x5a]);
        assert_eq!(format!("{:.3}", read!(tape)), "1.017");
    }
}
