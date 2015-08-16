use Result;
use band::{Band, Value};

pub type Real = f64;

impl Value for Real {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        macro_rules! bad(() => (raise!("found a malformed real number")));
        if try!(band.take::<u8>()) != 0x1e {
            bad!();
        }
        let mut buffer = String::new();
        let mut byte = 0;
        let mut high = true;
        loop {
            let nibble = match high {
                true => {
                    byte = try!(band.take::<u8>());
                    byte >> 4
                },
                false => byte & 0x0f,
            };
            high = !high;
            match nibble {
                0...9 => buffer.push(('0' as u8 + nibble) as char),
                0x0a => buffer.push('.'),
                0x0b => buffer.push('e'),
                0x0c => buffer.push_str("e-"),
                0x0e => buffer.push('-'),
                0x0f => break,
                _ => bad!(),
            }
        }
        match buffer.parse() {
            Ok(value) => Ok(value),
            _ => bad!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use band::Value;
    use compact::primitive::Real;
    use std::io::Cursor;

    macro_rules! read(($band:expr) => (Real::read(&mut $band).unwrap()));

    #[test]
    fn read() {
        let mut band = Cursor::new(vec![0x1e, 0xe2, 0xa2, 0x5f, 0x0f]);
        assert_eq!(read!(band), -2.25);

        let mut band = Cursor::new(vec![0x1e, 0x0a, 0x14, 0x05, 0x41, 0xc3, 0xff, 0x0f]);
        assert!((read!(band) - 0.140541e-3).abs() < 1e-14);
    }
}
