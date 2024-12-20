use crate::Result;

const SCALE: f32 = (1 << 16) as f32;

pub fn read<T: crate::tape::Read>(tape: &mut T) -> Result<f32> {
    let first = tape.take::<u8>()?;
    Ok(match first {
        0x20..=0xf6 => (first as i32 - 139) as f32,
        0xf7..=0xfa => ((first as i32 - 247) * 256 + tape.take::<u8>()? as i32 + 108) as f32,
        0xfb..=0xfe => (-(first as i32 - 251) * 256 - tape.take::<u8>()? as i32 - 108) as f32,
        0x1c => tape.take::<u16>()? as i16 as i32 as f32,
        0xff => (tape.take::<i32>()? as f32) / SCALE,
        _ => raise!("found a malformed number"),
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn real() {
        let mut tape = Cursor::new(vec![0xff, 0x00, 0x01, 0x04, 0x5a]);
        assert!(format!("{:.3}", super::read(&mut tape).unwrap()) == "1.017");
    }
}
