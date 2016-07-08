use Result;
use tape::{Tape, Value, Walue};

macro_rules! implement {
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

implement!(u8, 1);
implement!(u16, 2);
implement!(u32, 4);

impl Walue<usize> for Vec<u8> {
    fn read<T: Tape>(tape: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        unsafe { values.set_len(count) };
        fill!(tape, count, values);
        Ok(values)
    }
}