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
    ($name:ident, 1) => (impl Value for $name {
        fn read<T: Band>(band: &mut T) -> Result<Self> {
            Ok(read!(band, 1))
        }
    });
    ($name:ident, $size:expr) => (impl Value for $name {
        fn read<T: Band>(band: &mut T) -> Result<Self> {
            Ok($name::from_be(read!(band, $size)))
        }
    });
}

implement!(u8, 1);
implement!(u16, 2);
implement!(u32, 4);

impl Walue for Vec<u8> {
    fn read<T: Band>(band: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        unsafe { values.set_len(count) };
        fill!(band, count, values);
        Ok(values)
    }
}

mod offset;
mod real;
