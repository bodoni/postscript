use setup;

macro_rules! operations(
    ($($operator:ident => [$($number:expr),*],)*) => ({
        use postscript::compact::{Number, Operator};
        use std::collections::HashMap;
        let mut operations = HashMap::new();
        $(operations.insert(Operator::$operator, vec![$(Number::Integer($number)),*]);)*
        operations
    });
);

#[test]
fn charsets() {
    use postscript::compact::Charset;

    let set = setup();
    let vector = &set.charsets;

    assert_eq!(vector.len(), 1);
    match &vector[0] {
        &Charset::Format1(..) => {},
        _ => unreachable!(),
    }
}

#[test]
fn char_strings() {
    let set = setup();
    let vector = &set.char_strings;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].len(), 547);
}

#[test]
fn encodings() {
    use postscript::compact::Encoding;

    let set = setup();
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
fn global_subroutines() {
    let set = setup();
    let index = &set.global_subroutines;

    assert_eq!(index.len(), 181);
}

#[test]
fn header() {
    let set = setup();
    let table = &set.header;

    assert_eq!(table.major, 1);
    assert_eq!(table.minor, 0);
    assert_eq!(table.header_size, 4);
    assert_eq!(table.offset_size, 2);
}

#[test]
fn local_subroutines() {
    let set = setup();
    let vector = &set.local_subroutines;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].len(), 180);
}

#[test]
fn names() {
    let set = setup();
    let vector = &set.names;

    assert_eq!(vector.len(), 1);
    assert_eq!(&vector[0], "SourceSerifPro-Regular");
}

#[test]
fn private_dictionaries() {
    use postscript::compact::{Number, Operator};

    let set = setup();
    let vector = &set.private_dictionaries;

    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].len(), 13);
    assert_eq!(vector[0].get(Operator::BlueScale).unwrap(), &[Number::Real(0.0375)]);
}

#[test]
fn strings() {
    let set = setup();
    let index = &set.strings;

    assert_eq!(index.len(), 322);
    assert_eq!(index.get(175).unwrap(), "Aring");
    assert_eq!(index.get(500).unwrap(), "nine.tosf");
}

#[test]
fn top_dictionaries() {
    let set = setup();
    let vector = &set.top_dictionaries;

    assert_eq!(vector.len(), 1);
    assert_eq!(&*vector[0], &operations!(
        Version => [709],
        Notice => [710],
        Copyright => [711],
        FullName => [712],
        FamilyName => [712],
        Weight => [388],
        FontBBox => [-178, -335, 1138, 918],
        Charset => [8340],
        CharStrings => [8917],
        Private => [65, 33671],
    ));
}
