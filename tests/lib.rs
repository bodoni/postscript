extern crate postscript;

use postscript::Compact;
use postscript::compact::Header;
use std::io::Cursor;

#[test]
fn cff_header() {
    let compact = Compact::read(&mut read()).unwrap();
    assert_eq!(compact.header, Header { major: 1, minor: 0, hdrSize: 4, offSize: 2 });
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
