use setup;

macro_rules! operations(
    ($($operator:ident: [$($operand:expr),*],)*) => ({
        use postscript::compact::{Operand, Operator};
        use std::collections::HashMap;
        let mut operations = HashMap::new();
        $(operations.insert(Operator::$operator, vec![$($operand as Operand),*]);)*
        operations
    });
);

#[test]
fn char_sets() {
    use postscript::compact::CharSet;

    let set = setup();
    let vector = &set.char_sets;
    assert_eq!(vector.len(), 1);
    match &vector[0] {
        &CharSet::Format1(..) => {},
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
            assert_eq!(ok!(strings.get(ok!(encoding.get(42)))), "asterisk");
        },
        _ => unreachable!(),
    }
}

#[test]
fn global_dictionaries() {
    let set = setup();
    let vector = &set.global_dictionaries;
    assert_eq!(vector.len(), 1);
    assert_eq!(&*vector[0], &operations!(
        Version: [709],
        Notice: [710],
        Copyright: [711],
        FullName: [712],
        FamilyName: [712],
        Weight: [388],
        FontBBox: [-178, -335, 1138, 918],
        CharSet: [8340],
        CharStrings: [8917],
        Private: [65, 33671],
    ));
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
fn local_dictionaries() {
    let set = setup();
    let vector = &set.local_dictionaries;
    assert_eq!(vector.len(), 1);
    assert_eq!(&*vector[0], &operations!(
        DefaultWidthX: [370],
        FamilyOtherBlues: [-249, 10],
        BlueValues: [-20, 20, 473, 18, 34, 15, 104, 15, 10, 20, 40, 20],
        StemSnapH: [41, 15],
        StdHW: [41],
        NominalWidthX: [604],
        StdVW: [85],
        OtherBlues: [-249, 10],
        BlueFuzz: [0],
        Subrs: [65],
        FamilyBlues: [-20, 20, 473, 18, 34, 15, 104, 15, 10, 20, 40, 20],
        BlueScale: [0.0375],
        StemSnapV: [85, 10],
    ));
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
fn strings() {
    let set = setup();
    let index = &set.strings;
    assert_eq!(index.len(), 322);
    assert_eq!(ok!(index.get(175)), "Aring");
    assert_eq!(ok!(index.get(500)), "nine.tosf");
}
