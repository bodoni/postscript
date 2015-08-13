use std::io::{Read, Seek};

use Result;

#[doc(hidden)]
pub trait Band: Read + Seek {
}

#[doc(hidden)]
pub trait Value {
    fn read<T: Band>(&mut T) -> Result<Self>;
}

impl<T: Read + Seek> Band for T {
}
