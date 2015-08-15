//! Primitive data types.

use std::io::Read;
use std::mem;

use Result;
use band::{Band, Value, Walue};

pub type Card8 = u8;
pub type Card16 = u16;
pub type OffSize = u8;
pub type Offset = usize;

macro_rules! fill(
    ($band:ident, $count:expr, $buffer:ident) => (
        if try!($band.read(&mut $buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
    );
);

macro_rules! read(
    ($band:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = mem::uninitialized();
        fill!($band, $size, buffer);
        mem::transmute(buffer)
    });
);

macro_rules! implement {
    ($name:ident, 1) => (
        impl Value for $name {
            fn read<T: Band>(band: &mut T) -> Result<Self> {
                Ok(read!(band, 1))
            }
        }
    );
    ($name:ident, $size:expr) => (
        impl Value for $name {
            fn read<T: Band>(band: &mut T) -> Result<Self> {
                Ok($name::from_be(read!(band, $size)))
            }
        }
    );
}

implement!(u8, 1);
implement!(u16, 2);
implement!(u32, 4);

impl Value for f64 {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let mut buffer = String::new();
        let mut byte = 0;
        let mut high = true;
        loop {
            let nibble = match high {
                true => {
                    byte = try!(u8::read(band));
                    byte >> 4
                },
                false => byte & 0xf,
            };
            high = !high;
            match nibble {
                0...9 => buffer.push(('0' as u8 + nibble) as char),
                0xa => buffer.push('.'),
                0xb => buffer.push('e'),
                0xc => buffer.push_str("e-"),
                0xe => buffer.push('-'),
                0xf => break,
                _ => raise!("found a malformed real number"),
            }
        }
        Ok(match buffer.parse() {
            Ok(value) => value,
            _ => raise!("found a malformed real number"),
        })
    }
}

#[cfg(target_endian = "big")]
macro_rules! assemble(
    ($hi:expr, $me:expr, $lo:expr) => ([0, $hi, $me, $lo]);
);

#[cfg(target_endian = "little")]
macro_rules! assemble(
    ($hi:expr, $me:expr, $lo:expr) => ([$lo, $me, $hi, 0]);
);

impl Walue for usize {
    fn read<T: Band>(band: &mut T, size: usize) -> Result<Self> {
        Ok(match size {
            1 => try!(u8::read(band)) as usize,
            2 => try!(u16::read(band)) as usize,
            3 => {
                let trio: [u8; 3] = read!(band, 3);
                unsafe { mem::transmute::<_, u32>(assemble!(trio[0], trio[1], trio[2])) as usize }
            },
            4 => try!(u32::read(band)) as usize,
            _ => raise!("found an invalid size"),
        })
    }
}

impl Walue for Vec<u8> {
    fn read<T: Band>(band: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        unsafe { values.set_len(count) };
        fill!(band, count, values);
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use band::{Value, Walue};
    use std::io::Cursor;

    #[test]
    fn value_read_f64() {
        let mut band = Cursor::new(vec![0xe2, 0xa2, 0x5f, 0x0f]);
        assert_eq!(f64::read(&mut band).unwrap(), -2.25);

        let mut band = Cursor::new(vec![0x0a, 0x14, 0x05, 0x41, 0xc3, 0xff, 0x0f]);
        assert!((f64::read(&mut band).unwrap() - 0.140541e-3).abs() < 1e-14);
    }

    #[test]
    fn walue_read_usize() {
        let mut band = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        assert_eq!(usize::read(&mut band, 1).unwrap(), 0x01);
        assert_eq!(usize::read(&mut band, 2).unwrap(), 0x0203);
        assert_eq!(usize::read(&mut band, 3).unwrap(), 0x040506);
        assert_eq!(usize::read(&mut band, 4).unwrap(), 0x0708090a);
    }
}
