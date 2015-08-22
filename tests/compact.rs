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
fn names() {
    let set = FontSet::read(&mut read()).unwrap();
    let index = &set.names;

    assert_eq!(index.count, 1);
    assert_eq!(index.offSize, 1);
    assert_eq!(index.offset, &[1, 23]);
    assert_eq!(index.get(0).unwrap(), "SourceSerifPro-Regular");
}

#[test]
fn dictionaries() {
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
    let index = &set.dictionaries;

    assert_eq!(index.count, 1);
    assert_eq!(index.offSize, 1);
    assert_eq!(index.offset, &[1, 45]);
    assert_eq!(&*index.get(0).unwrap().unwrap(), &operations!(
        version => [Integer(709)], Notice => [Integer(710)], Copyright => [Integer(711)],
        FullName => [Integer(712)], FamilyName => [Integer(712)], Weight => [Integer(388)],
        FontBBox => [Integer(-178), Integer(-335), Integer(1138), Integer(918)],
        charset => [Integer(8340)], CharStrings => [Integer(8917)],
        Private => [Integer(65), Integer(33671)],
    ));
}

#[test]
fn strings() {
    let set = FontSet::read(&mut read()).unwrap();
    let index = &set.strings;

    assert_eq!(index.count, 322);
    assert_eq!(index.offSize, 2);
    assert_eq!(index.get(175).unwrap(), "Aring");
    assert_eq!(index.get(500).unwrap(), "nine.tosf");
}

#[test]
fn subroutines() {
    let set = FontSet::read(&mut read()).unwrap();
    let index = &set.subroutines;

    assert_eq!(index.count, 181);
    assert_eq!(index.offSize, 2);
}

#[test]
fn encodings() {
    use postscript::compact::compound::Encoding;

    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.encodings;
    let strings = &set.strings;

    assert_eq!(vector.len(), 1);
    match &vector[0] {
        encoding @ &Encoding::Standard => {
            assert_eq!(strings.get(encoding.get(42).unwrap()).unwrap(), "asterisk");
        },
        _ => unreachable!(),
    }
}

#[test]
fn charsets() {
    use postscript::compact::compound::Charset;

    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.charsets;

    assert_eq!(vector.len(), 1);
    match &vector[0] {
        &Charset::Format1(..) => {},
        _ => unreachable!(),
    }
}

#[test]
fn char_strings() {
    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.char_strings;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].count, 547);
    assert_eq!(vector[0].offSize, 2);
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
