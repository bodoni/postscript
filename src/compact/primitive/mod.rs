//! Primitive data types.

use std::mem;

use Result;
use band::{Band, ParametrizedValue, Value};

pub type GlyphID = u16;
pub type OffsetSize = u8;
pub type StringID = u16;

macro_rules! fill(
    ($band:ident, $count:expr, $buffer:ident) => (
        if try!(::std::io::Read::read($band, &mut $buffer)) != $count {
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

impl ParametrizedValue<usize> for Vec<u8> {
    fn read<T: Band>(band: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        unsafe { values.set_len(count) };
        fill!(band, count, values);
        Ok(values)
    }
}

mod number;
mod offset;

pub use self::number::Number;
pub use self::offset::Offset;
