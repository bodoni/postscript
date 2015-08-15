use Result;
use band::{Band, Value};

impl Value for f64 {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let mut buffer = String::new();
        let mut byte = 0;
        let mut high = true;
        loop {
            let nibble = match high {
                true => {
                    byte = try!(u8::read(band));
                    byte >> 4
                },
                false => byte & 0xf,
            };
            high = !high;
            match nibble {
                0...9 => buffer.push(('0' as u8 + nibble) as char),
                0xa => buffer.push('.'),
                0xb => buffer.push('e'),
                0xc => buffer.push_str("e-"),
                0xe => buffer.push('-'),
                0xf => break,
                _ => raise!("found a malformed real number"),
            }
        }
        Ok(match buffer.parse() {
            Ok(value) => value,
            _ => raise!("found a malformed real number"),
        })
    }
}

#[cfg(test)]
mod tests {
    use band::Value;
    use std::io::Cursor;

    #[test]
    fn read() {
        let mut band = Cursor::new(vec![0xe2, 0xa2, 0x5f, 0x0f]);
        assert_eq!(f64::read(&mut band).unwrap(), -2.25);

        let mut band = Cursor::new(vec![0x0a, 0x14, 0x05, 0x41, 0xc3, 0xff, 0x0f]);
        assert!((f64::read(&mut band).unwrap() - 0.140541e-3).abs() < 1e-14);
    }
}
