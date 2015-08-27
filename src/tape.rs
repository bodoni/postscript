use std::io::{Read, Seek, SeekFrom};

use Result;

#[doc(hidden)]
pub trait Tape: Read + Seek + Sized {
    fn count(&mut self) -> Result<u64> {
        let current = try!(self.position());
        let end = self.seek(SeekFrom::End(0));
        try!(self.jump(current));
        end
    }

    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    fn peek<T: Value>(&mut self) -> Result<T> {
        let current = try!(self.position());
        let result = Value::read(self);
        try!(self.jump(current));
        result
    }

    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    #[inline(always)]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }
}

#[doc(hidden)]
pub trait Value: Sized {
    fn read<T: Tape>(&mut T) -> Result<Self>;
}

#[doc(hidden)]
pub trait ValueExt<P>: Sized {
    fn read<T: Tape>(&mut T, P) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {
}
