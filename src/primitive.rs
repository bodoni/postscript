//! Primitive data types.

use std::mem;

use Result;
use band::{Band, Value};

pub type Card8 = u8;
pub type Card16 = u16;
pub type OffSize = u8;

macro_rules! read(
    ($band:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = mem::uninitialized();
        if try!($band.read(&mut buffer)) != $size {
            return raise!("failed to read as much as needed");
        }
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
