#[macro_use]
mod support;

macro_rules! operations(
    ($($operator:ident: [$($operand:expr),*],)*) => ({
        use postscript::type2::{Operand, Operator};
        let mut operations = vec![];
        $(operations.push((Operator::$operator, vec![$($operand as Operand),*]));)*
        operations
    });
);

mod source_serif {
    use postscript::compact1::font_set::Record;
    use postscript::type2::Program;

    use crate::support::{setup_font_set, Fixture};

    #[test]
    fn program_all() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let global = &set.subroutines;
        let local = match &set.records[0] {
            Record::CharacterNameKeyed(ref record) => &*record.subroutines,
            _ => unreachable!(),
        };
        for code in set.character_strings[0].iter() {
            let mut program = Program::new(code, global, local);
            while let Some(..) = ok!(program.next()) {}
        }
    }

    #[test]
    fn program_one() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let code = &set.character_strings[0][134];
        let global = &set.subroutines;
        let local = match &set.records[0] {
            Record::CharacterNameKeyed(ref record) => &*record.subroutines,
            _ => unreachable!(),
        };
        let mut program = Program::new(code, global, local);
        let mut operations = vec![];
        while let Some(operation) = ok!(program.next()) {
            operations.push(operation);
        }
        assert_eq!(program.width(), Some(-95.0));
        assert_eq!(
            operations,
            operations!(
                HStemHM: [-15, 66, -61, 52, 403, 46, 82, 63, 20, 62],
                HintMask: [],
                RMoveTo: [112, 585],
                VHCurveTo: [50, 20, 21, 28, 21, 16, -13, -26, 27],
                HintMask: [],
                HHCurveTo: [-29, 29, 26, -15, 31],
                HVCurveTo: [53, 43, 42, 68, 10, -1, 7, -1, 7],
                HLineTo: [-34],
                VHCurveTo: [-51, -21, -20, -26, -21, -18, 13, 26, -26],
                HintMask: [],
                HHCurveTo: [28, -29, -26, 15, -31],
                HVCurveTo: [-53, -43, -42, -68, -7, 1, -10, 1, -6],
                RMoveTo: [246, -479],
                HintMask: [],
                HHCurveTo: [-41, -58, -19, -14, -33],
                HVCurveTo: [-24, -21, 7, 16, -15],
                VVCurveTo: [-12, 12, -8, 15, 26],
                VHCurveTo: [30, 11, 39, 87, 34],
                RRCurveTo: [21, 8, 36, 12, 35, 10],
                HintMask: [],
                RMoveTo: [159, -196],
                RLineTo: [-5, -5],
                HHCurveTo: [-8, -8, -13, -9, -16],
                HVCurveTo: [-22, -11, 18, 41],
                VLineTo: [216],
                VHCurveTo: [126, -50, 48, -105, -102, -74, -50, -76, -19],
                HHCurveTo: [-26, 3, 16, -17, 28],
                HVCurveTo: [27, 18, 17, 31, 9],
                RLineTo: [20, 69],
                HHCurveTo: [5, 20, 17, 1, 14],
                HVCurveTo: [66, 28, -24, -97],
                VLineTo: [-27],
                RRCurveTo: [-40, -9, -42, -13, -31, -11],
                VVCurveTo: [-135, -49, -31, -46, -57],
                HintMask: [],
                VHCurveTo: [-83, 61, -44, 73, 59, 33, 27, 55, 55],
                HintMask: [],
                HHCurveTo: [-47, 8, 31, -30, 57],
                HVCurveTo: [32, 26, 10, 42, 23],
            )
        );
    }
}
