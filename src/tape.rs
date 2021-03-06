use std::io::{Read, Seek, SeekFrom};

use crate::Result;

/// A type that can read.
pub trait Tape: Read + Seek + Sized {
    /// Read a value.
    #[inline]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }

    /// Read a value given a parameter.
    #[inline]
    fn take_given<T: Walue>(&mut self, parameter: T::Parameter) -> Result<T> {
        Walue::read(self, parameter)
    }

    #[doc(hidden)]
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[doc(hidden)]
    fn peek<T: Value>(&mut self) -> Result<T> {
        let current = self.position()?;
        let result = Value::read(self);
        self.jump(current)?;
        result
    }

    #[doc(hidden)]
    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

/// A type that can be read.
pub trait Value: Sized {
    /// Read a value.
    fn read<T: Tape>(tape: &mut T) -> Result<Self>;
}

/// A type that can be read given a parameter.
pub trait Walue: Sized {
    /// The parameter type.
    type Parameter;

    /// Read a value.
    fn read<T: Tape>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {}

macro_rules! read(
    ($tape:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = ::std::mem::zeroed();
        ::std::io::Read::read_exact($tape, &mut buffer)?;
        ::std::mem::transmute(buffer)
    });
);

macro_rules! value {
    ([$kind:ident; $count:expr], 1) => {
        impl Value for [$kind; $count] {
            #[inline]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, $count))
            }
        }
    };
    ($kind:ident, 1) => {
        impl Value for $kind {
            #[inline]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, 1))
            }
        }
    };
    ($kind:ident, $size:expr) => {
        impl Value for $kind {
            #[inline]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok($kind::from_be(read!(tape, $size)))
            }
        }
    };
}

value!(u8, 1);
value!(u16, 2);
value!(u32, 4);
value!([u8; 3], 1);

macro_rules! walue {
    ($kind:ty, 1) => {
        impl Walue for $kind {
            type Parameter = usize;

            fn read<T: Tape>(tape: &mut T, count: usize) -> Result<Self> {
                let mut buffer = vec![0; count];
                ::std::io::Read::read_exact(tape, &mut buffer)?;
                Ok(buffer)
            }
        }
    };
}

walue!(Vec<u8>, 1);
