use postscript::compact::FontSet;
use std::io::Cursor;

#[test]
fn header() {
    let set = FontSet::read(&mut read()).unwrap();
    let table = &set.header;

    assert_eq!(table.major, 1);
    assert_eq!(table.minor, 0);
    assert_eq!(table.hdrSize, 4);
    assert_eq!(table.offSize, 2);
}

#[test]
fn name_index() {
    let set = FontSet::read(&mut read()).unwrap();
    let table = &set.name_index;

    assert_eq!(table.count, 1);
    assert_eq!(table.offSize, 1);
    assert_eq!(table.offset, &[1, 23]);
    assert_eq!(table.get(0).unwrap(), "SourceSerifPro-Regular");
}

#[test]
fn top_dictionary() {
    use postscript::compact::compound::{Operand, Operator};
    use std::collections::HashMap;

    macro_rules! operations(
        ($($operator:ident => [$($operand:ident($number:expr)),*],)*) => ({
            let mut operations = HashMap::new();
            $(operations.insert(Operator::$operator, vec![$(Operand::$operand($number)),*]);)*
            operations
        });
    );

    let set = FontSet::read(&mut read()).unwrap();
    let table = &set.top_dictionary;

    assert_eq!(table.count, 1);
    assert_eq!(table.offSize, 1);
    assert_eq!(table.offset, &[1, 45]);
    assert_eq!(table.get(0).unwrap().unwrap(), operations!(
        version => [Integer(709)], Notice => [Integer(710)], Copyright => [Integer(711)],
        FullName => [Integer(712)], FamilyName => [Integer(712)], Weight => [Integer(388)],
        FontBBox => [Integer(-178), Integer(-335), Integer(1138), Integer(918)],
        charset => [Integer(8340)], CharStrings => [Integer(8917)],
        Private => [Integer(65), Integer(33671)],
    ));
}

#[test]
fn string_index() {
    let set = FontSet::read(&mut read()).unwrap();
    let table = &set.string_index;

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
