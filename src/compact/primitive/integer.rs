use Result;
use band::{Band, Value};

pub type Integer = i32;

impl Value for Integer {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let byte0 = try!(u8::read(band));
        Ok(match byte0 {
            32...246 => byte0 as Integer - 139,
            247...250 => (byte0 as Integer - 247) * 256 + try!(u8::read(band)) as Integer + 108,
            251...254 => -(byte0 as Integer - 251) * 256 - try!(u8::read(band)) as Integer - 108,
            28 => try!(u16::read(band)) as i16 as Integer,
            29 => try!(u32::read(band)) as i32 as Integer,
            _ => raise!("found a malformed integer"),
        })
    }
}

#[cfg(test)]
mod tests {
    use band::Value;
    use compact::primitive::Integer;
    use std::io::Cursor;

    #[test]
    fn read() {
        let mut band = Cursor::new(vec![0x8b]);
        assert_eq!(Integer::read(&mut band).unwrap(), 0);

        let mut band = Cursor::new(vec![0xef]);
        assert_eq!(Integer::read(&mut band).unwrap(), 100);

        let mut band = Cursor::new(vec![0x27]);
        assert_eq!(Integer::read(&mut band).unwrap(), -100);

        let mut band = Cursor::new(vec![0xfa, 0x7c]);
        assert_eq!(Integer::read(&mut band).unwrap(), 1000);

        let mut band = Cursor::new(vec![0xfe, 0x7c]);
        assert_eq!(Integer::read(&mut band).unwrap(), -1000);

        let mut band = Cursor::new(vec![0x1c, 0x27, 0x10]);
        assert_eq!(Integer::read(&mut band).unwrap(), 10000);

        let mut band = Cursor::new(vec![0x1c, 0xd8, 0xf0]);
        assert_eq!(Integer::read(&mut band).unwrap(), -10000);

        let mut band = Cursor::new(vec![0x1d, 0x00, 0x01, 0x86, 0xa0]);
        assert_eq!(Integer::read(&mut band).unwrap(), 100000);

        let mut band = Cursor::new(vec![0x1d, 0xff, 0xfe, 0x79, 0x60]);
        assert_eq!(Integer::read(&mut band).unwrap(), -100000);
    }
}
