extern crate postscript;

use postscript::Compact;
use std::io::Cursor;

#[test]
fn cff_header() {
    let compact = Compact::read(&mut read()).unwrap();
    assert_eq!(compact.header.major, 1);
    assert_eq!(compact.header.minor, 0);
}

fn read() -> Cursor<Vec<u8>> {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    let mut file = File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap();
    file.seek(SeekFrom::Start(17732)).unwrap();
    let mut buffer = vec![0; 37728];
    assert_eq!(file.read(&mut buffer).unwrap(), buffer.len());
    Cursor::new(buffer)
}
