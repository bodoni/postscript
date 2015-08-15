use std::io::{Read, Seek, SeekFrom};

use Result;

#[doc(hidden)]
pub trait Band: Read + Seek {
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }
}

#[doc(hidden)]
pub trait Value {
    fn read<T: Band>(&mut T) -> Result<Self>;
}

#[doc(hidden)]
pub trait Walue {
    fn read<T: Band>(&mut T, usize) -> Result<Self>;
}

impl<T: Read + Seek> Band for T {
}
