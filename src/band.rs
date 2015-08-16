use std::io::{Read, Seek, SeekFrom};

use Result;

#[doc(hidden)]
pub trait Band: Read + Seek + Sized {
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[inline]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }

    fn peek<T: Value>(&mut self) -> Result<T> {
        let position = try!(self.position());
        let result = Value::read(self);
        try!(self.jump(position));
        Ok(try!(result))
    }

    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

#[doc(hidden)]
pub trait Value {
    fn read<T: Band>(&mut T) -> Result<Self>;
}

#[doc(hidden)]
pub trait ParametrizedValue<P> {
    fn read<T: Band>(&mut T, P) -> Result<Self>;
}

impl<T: Read + Seek> Band for T {
}
