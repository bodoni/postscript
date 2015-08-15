use postscript::Compact;
use std::io::Cursor;

#[test]
fn header() {
    let compact = Compact::read(&mut read()).unwrap();
    let table = &compact.header;

    assert_eq!(table.major, 1);
    assert_eq!(table.minor, 0);
    assert_eq!(table.hdrSize, 4);
    assert_eq!(table.offSize, 2);
}

#[test]
fn name_index() {
    let compact = Compact::read(&mut read()).unwrap();
    let table = &compact.name_index;

    assert_eq!(table.count, 1);
    assert_eq!(table.offSize, 1);
    assert_eq!(table.offset, &[1, 23]);
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
