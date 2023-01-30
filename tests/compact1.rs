extern crate postscript;

#[macro_use]
mod support;

macro_rules! expand(
    ($operations:expr) => (
        $operations.iter().map(|operation| (&operation.0, &operation.1)).collect::<Vec<_>>()
    );
);

macro_rules! operations(
    ($($operator:ident: [$($operand:expr),*],)*) => ({
        use postscript::compact1::{Operation, Operations, Operator};
        Operations(vec![$(Operation(Operator::$operator, vec![$($operand.into()),*]),)*])
    });
);

mod noto_sans_direct {
    use postscript::Tape;

    use crate::support::{setup, Fixture};

    #[test]
    fn header() {
        use postscript::compact1::Header;

        let mut tape = setup(Fixture::NotoSansJP);
        let table = ok!(tape.take::<Header>());
        assert_eq!(table.major, 1);
        assert_eq!(table.minor, 0);
        assert_eq!(table.header_size, 4);
        assert_eq!(table.offset_size, 4);
    }

    #[test]
    fn names() {
        use postscript::compact1::index::Names;
        use postscript::compact1::Header;

        let mut tape = setup(Fixture::NotoSansJP);
        let position = ok!(tape.position());
        let table = ok!(tape.take::<Header>());
        ok!(tape.jump(position + table.header_size as u64));
        let table: Vec<_> = ok!(ok!(tape.take::<Names>()).try_into());
        assert_eq!(table.len(), 1);
        assert_eq!(&table[0], "NotoSansJP-Regular");
    }

    #[test]
    fn operations() {
        use postscript::compact1::index::{Dictionaries, Names};
        use postscript::compact1::Header;

        let mut tape = setup(Fixture::NotoSansJP);
        let position = ok!(tape.position());
        let table = ok!(tape.take::<Header>());
        ok!(tape.jump(position + table.header_size as u64));
        let _ = ok!(tape.take::<Names>());
        let table: Vec<_> = ok!(ok!(tape.take::<Dictionaries>()).try_into());
        assert_eq!(table.len(), 1);
        let operations = operations!(
            ROS: [394, 395, 0],
            Notice: [391],
            FullName: [392],
            FamilyName: [393],
            Weight: [388],
            UnderlinePosition: [-150],
            FontBBox: [-1002, -1048, 2928, 1808],
            CIDFontVersion: [2.002],
            CIDCount: [65529],
            CharSet: [7068],
            CharStrings: [43287],
            FDSelect: [42687],
            FDArray: [43013],
        );
        assert_eq!(expand!(table[0].0), expand!(operations.0));
    }
}

mod noto_sans_indirect {
    use crate::support::{setup_font_set, Fixture};

    #[test]
    fn char_strings() {
        let set = setup_font_set(Fixture::NotoSansJP);
        let tables = &set.char_strings;
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].len(), 17810);
    }

    #[test]
    fn records() {
        use postscript::compact1::font_set::character_id_keyed::Encoding;
        use postscript::compact1::font_set::Record;
        use postscript::compact1::{Number, Operator};

        let set = setup_font_set(Fixture::NotoSansJP);
        let records = &set.records;
        let strings = &set.strings;
        assert_eq!(records.len(), 1);
        let record = match &records[0] {
            Record::CharacterIDKeyed(ref record) => record,
            _ => unreachable!(),
        };
        assert_eq!(ok!(strings.get(record.registry)), "Adobe");
        assert_eq!(ok!(strings.get(record.ordering)), "Identity");
        assert_eq!(record.supplement, Number::Integer(0));
        match record.encoding {
            Encoding::Format3(ref encoding) => {
                assert_eq!(encoding.range_count, 107);
                assert_eq!(encoding.glyph_count, 17810);
            }
            _ => unreachable!(),
        }
        assert_eq!(record.operations.len(), 18);
        let operations = operations!(
            FontName: [396],
            Private: [32, 3978751],
        );
        assert_eq!(expand!(record.operations[0].0), expand!(operations.0));
        assert_eq!(
            ok!(strings.get(ok!(record.operations[0][0].1[0].try_into()))),
            "NotoSansJP-Regular-Alphabetic",
        );
        assert_eq!(record.records.len(), 18);
        assert_eq!(
            record
                .records
                .iter()
                .filter(|record| record.operations.get(Operator::Subrs).is_some())
                .count(),
            14,
        );
        assert_eq!(
            record
                .records
                .iter()
                .filter(|record| record.subroutines.len() > 0)
                .count(),
            14,
        );
        let operations = operations!(
            BlueValues: [-13, 13, 544, 13, 178, 12],
            OtherBlues: [-250, 21],
            StdHW: [78],
            StdVW: [85],
            StemSnapH: [78, 33],
            StemSnapV: [85, 10],
            DefaultWidthX: [1000],
            Subrs: [32],
        );
        assert_eq!(
            expand!(record.records[0].operations.0),
            expand!(operations.0),
        );
        let operations = operations!(
            BlueValues: [-250, 0, 1350, 0],
            StdHW: [78],
            StdVW: [61],
            LanguageGroup: [1],
            DefaultWidthX: [500],
        );
        assert_eq!(
            expand!(record.records[7].operations.0),
            expand!(operations.0),
        );
    }
}

mod source_serif {
    use crate::support::{setup_font_set, Fixture};

    #[test]
    fn char_sets() {
        use postscript::compact1::CharSet;

        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.char_sets;
        assert_eq!(tables.len(), 1);
        match &tables[0] {
            &CharSet::Format1(..) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn char_strings() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.char_strings;
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].len(), 547);
    }

    #[test]
    fn encodings() {
        use postscript::compact1::Encoding;

        let set = setup_font_set(Fixture::SourceSerifPro);
        let encodings = &set.encodings;
        let strings = &set.strings;
        assert_eq!(encodings.len(), 1);
        match &encodings[0] {
            encoding @ &Encoding::Standard => {
                assert_eq!(ok!(strings.get(ok!(encoding.get(0)))), ".notdef");
                assert_eq!(ok!(strings.get(ok!(encoding.get(42)))), "asterisk");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn header() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.header;
        assert_eq!(table.major, 1);
        assert_eq!(table.minor, 0);
        assert_eq!(table.header_size, 4);
        assert_eq!(table.offset_size, 2);
    }

    #[test]
    fn names() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table: Vec<_> = ok!(set.names.try_into());
        assert_eq!(table.len(), 1);
        assert_eq!(&table[0], "SourceSerifPro-Regular");
    }

    #[test]
    fn operations() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.operations;
        assert_eq!(table.len(), 1);
        let operations = operations!(
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
        );
        assert_eq!(expand!(table[0].0), expand!(operations.0));
    }

    #[test]
    fn records() {
        use postscript::compact1::font_set::Record;

        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.records;
        assert_eq!(tables.len(), 1);
        let operations = operations!(
            BlueValues: [-20, 20, 473, 18, 34, 15, 104, 15, 10, 20, 40, 20],
            OtherBlues: [-249, 10],
            FamilyBlues: [-20, 20, 473, 18, 34, 15, 104, 15, 10, 20, 40, 20],
            FamilyOtherBlues: [-249, 10],
            BlueScale: [0.0375],
            BlueFuzz: [0],
            StdHW: [41],
            StdVW: [85],
            StemSnapH: [41, 15],
            StemSnapV: [85, 10],
            DefaultWidthX: [370],
            NominalWidthX: [604],
            Subrs: [65],
        );
        match &tables[0] {
            Record::CharacterNameKeyed(ref record) => {
                assert_eq!(expand!(record.operations.0), expand!(operations.0));
                assert_eq!(record.subroutines.len(), 180);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn strings() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.strings;
        assert_eq!(table.len(), 322);
        assert_eq!(ok!(table.get(0)), ".notdef");
        assert_eq!(ok!(table.get(175)), "Aring");
        assert_eq!(ok!(table.get(500)), "nine.tosf");
    }

    #[test]
    fn subroutines() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.subroutines;
        assert_eq!(table.len(), 181);
    }
}
