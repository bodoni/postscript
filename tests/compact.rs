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
    assert_eq!(table.get(0).unwrap(), "SourceSerifPro-Regular");
}

#[test]
fn dictionary_index() {
    use postscript::compact::compound::Operand::*;
    use postscript::compact::compound::Operation;

    let compact = Compact::read(&mut read()).unwrap();
    let table = &compact.dictionary_index;

    assert_eq!(table.count, 1);
    assert_eq!(table.offSize, 1);
    assert_eq!(table.offset, &[1, 45]);
    assert_eq!(table.get(0).unwrap().unwrap(), &[
        Operation(0, vec![Integer(709)]),
        Operation(1, vec![Integer(710)]),
        Operation(3072, vec![Integer(711)]),
        Operation(2, vec![Integer(712)]),
        Operation(3, vec![Integer(712)]),
        Operation(4, vec![Integer(388)]),
        Operation(5, vec![Integer(-178), Integer(-335), Integer(1138), Integer(918)]),
        Operation(15, vec![Integer(8340)]),
        Operation(17, vec![Integer(8917)]),
        Operation(18, vec![Integer(65), Integer(33671)]),
    ]);
}

#[test]
fn string_index() {
    let compact = Compact::read(&mut read()).unwrap();
    let table = &compact.string_index;

    assert_eq!(table.count, 322);
    assert_eq!(table.offSize, 2);
    assert_eq!(table.get(175).unwrap(), "Aring");
    assert_eq!(table.get(500).unwrap(), "nine.tosf");
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
