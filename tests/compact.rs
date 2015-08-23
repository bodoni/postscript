use postscript::compact::FontSet;

use read;

macro_rules! compact_operations(
    ($($operator:ident => [$($argument:ident($number:expr)),*],)*) => ({
        use postscript::compact::compound::Operator;
        use postscript::compact::primitive::Number;
        use std::collections::HashMap;
        let mut operations = HashMap::new();
        $(operations.insert(Operator::$operator, vec![$(Number::$argument($number)),*]);)*
        operations
    });
);

macro_rules! type2_operations(
    ($(($operator:ident, [$($argument:ident($number:expr)),*]),)*) => ({
        use postscript::type2::compound::Operator;
        use postscript::type2::primitive::Number;
        let mut operations = vec![];
        $(operations.push((Operator::$operator, vec![$(Number::$argument($number)),*]));)*
        operations
    });
);

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
    let vector = &set.names;

    assert_eq!(vector.len(), 1);
    assert_eq!(&vector[0], "SourceSerifPro-Regular");
}

#[test]
fn top_dictionaries() {
    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.top_dictionaries;

    assert_eq!(vector.len(), 1);
    assert_eq!(&*vector[0], &compact_operations!(
        Version => [Integer(709)], Notice => [Integer(710)], Copyright => [Integer(711)],
        FullName => [Integer(712)], FamilyName => [Integer(712)], Weight => [Integer(388)],
        FontBBox => [Integer(-178), Integer(-335), Integer(1138), Integer(918)],
        Charset => [Integer(8340)], Charstrings => [Integer(8917)],
        Private => [Integer(65), Integer(33671)],
    ));
}

#[test]
fn strings() {
    let set = FontSet::read(&mut read()).unwrap();
    let index = &set.strings;

    assert_eq!(index.len(), 322);
    assert_eq!(index.get(175).unwrap(), "Aring");
    assert_eq!(index.get(500).unwrap(), "nine.tosf");
}

#[test]
fn global_subroutines() {
    let set = FontSet::read(&mut read()).unwrap();
    let index = &set.global_subroutines;

    assert_eq!(index.len(), 181);
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
fn charstrings() {
    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.charstrings;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].len(), 547);
}

#[test]
fn private_dictionaries() {
    use postscript::compact::compound::Operator;
    use postscript::compact::primitive::Number;

    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.private_dictionaries;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].len(), 13);
    assert_eq!(vector[0].get(Operator::BlueScale).unwrap(), &[Number::Real(0.0375)]);
}

#[test]
fn local_subroutines() {
    let set = FontSet::read(&mut read()).unwrap();
    let vector = &set.local_subroutines;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].len(), 180);
}
