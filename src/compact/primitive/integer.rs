use Result;
use band::{Band, Value};

pub type Integer = i32;

impl Value for Integer {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        macro_rules! read(
            ($kind:ident, $via:ty) => (try!($kind::read(band)) as $via as Integer);
            ($kind:ident) => (try!($kind::read(band)) as Integer);
        );
        let byte0 = read!(u8);
        Ok(match byte0 {
            0x20...0xf6 => byte0 - 139,
            0xf7...0xfa => (byte0 - 247) * 256 + read!(u8) + 108,
            0xfb...0xfe => -(byte0 - 251) * 256 - read!(u8) - 108,
            0x1c => read!(u16, i16),
            0x1d => read!(u32, i32),
            _ => raise!("found a malformed integer"),
        })
    }
}

#[cfg(test)]
mod tests {
    use band::Value;
    use compact::primitive::Integer;
    use std::io::Cursor;

    macro_rules! read(($band:expr) => (Integer::read(&mut $band).unwrap()));

    #[test]
    fn read() {
        let mut band = Cursor::new(vec![0x8b]);
        assert_eq!(read!(band), 0);

        let mut band = Cursor::new(vec![0xef]);
        assert_eq!(read!(band), 100);

        let mut band = Cursor::new(vec![0x27]);
        assert_eq!(read!(band), -100);

        let mut band = Cursor::new(vec![0xfa, 0x7c]);
        assert_eq!(read!(band), 1000);

        let mut band = Cursor::new(vec![0xfe, 0x7c]);
        assert_eq!(read!(band), -1000);

        let mut band = Cursor::new(vec![0x1c, 0x27, 0x10]);
        assert_eq!(read!(band), 10000);

        let mut band = Cursor::new(vec![0x1c, 0xd8, 0xf0]);
        assert_eq!(read!(band), -10000);

        let mut band = Cursor::new(vec![0x1d, 0x00, 0x01, 0x86, 0xa0]);
        assert_eq!(read!(band), 100000);

        let mut band = Cursor::new(vec![0x1d, 0xff, 0xfe, 0x79, 0x60]);
        assert_eq!(read!(band), -100000);
    }
}
