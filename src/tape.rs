use std::io::{Read, Seek, SeekFrom};

use Result;

/// A type that can read.
pub trait Tape: Read + Seek + Sized {
    #[doc(hidden)]
    fn count(&mut self) -> Result<u64> {
        let current = try!(self.position());
        let end = self.seek(SeekFrom::End(0));
        try!(self.jump(current));
        end
    }

    #[doc(hidden)]
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[doc(hidden)]
    fn peek<T: Value>(&mut self) -> Result<T> {
        let current = try!(self.position());
        let result = Value::read(self);
        try!(self.jump(current));
        result
    }

    #[doc(hidden)]
    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    #[doc(hidden)]
    #[inline(always)]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }
}

/// A type that can be read.
pub trait Value: Sized {
    /// Read a value.
    fn read<T: Tape>(&mut T) -> Result<Self>;
}

/// A type that can be read provided a parameter.
pub trait Walue<P>: Sized {
    /// Read a value.
    fn read<T: Tape>(&mut T, P) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {}

macro_rules! value {
    ($name:ident, 1) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok(read!(tape, 1))
        }
    });
    ($name:ident, $size:expr) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok($name::from_be(read!(tape, $size)))
        }
    });
}

value!(u8, 1);
value!(u16, 2);
value!(u32, 4);

impl Walue<usize> for Vec<u8> {
    fn read<T: Tape>(tape: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        unsafe { values.set_len(count) };
        read!(tape, count, values);
        Ok(values)
    }
}
