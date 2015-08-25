use postscript::compact::FontSet;
use postscript::type2::Program;

use read;

macro_rules! operations(
    ($(($operator:ident, [$($argument:ident($number:expr)),*]),)*) => ({
        use postscript::type2::compound::Operator;
        use postscript::type2::primitive::Number;
        let mut operations = vec![];
        $(operations.push((Operator::$operator, vec![$(Number::$argument($number)),*]));)*
        operations
    });
);

#[test]
fn program() {
    let set = FontSet::read(&mut read()).unwrap();

    let code = &set.charstrings[0][134];
    let global = &set.global_subroutines;
    let local = &set.local_subroutines[0];

    let mut program = Program::new(code, global, local);
    let mut operations = vec![];
    while let Some(operation) = program.next().unwrap() {
        operations.push(operation);
    }

    assert_eq!(operations, operations![
        (HStemHM, [Integer(-95), Integer(-15), Integer(66), Integer(-61), Integer(52),
                   Integer(403), Integer(46), Integer(82), Integer(63), Integer(20), Integer(62)]),
        (HintMask, [Integer(45), Integer(89), Integer(-58), Integer(36), Integer(212), Integer(84),
                    Integer(-38), Integer(36)]),
        (RMoveTo, [Integer(112), Integer(585)]),
        (VHCurveTo, [Integer(50), Integer(20), Integer(21), Integer(28), Integer(21), Integer(16),
                     Integer(-13), Integer(-26), Integer(27)]),
        (HintMask, []),
        (HHCurveTo, [Integer(-29), Integer(29), Integer(26), Integer(-15), Integer(31)]),
        (HVCurveTo, [Integer(53), Integer(43), Integer(42), Integer(68), Integer(10), Integer(-1),
                     Integer(7), Integer(-1), Integer(7)]),
        (HLineTo, [Integer(-34)]),
        (VHCurveTo, [Integer(-51), Integer(-21), Integer(-20), Integer(-26), Integer(-21),
                     Integer(-18), Integer(13), Integer(26), Integer(-26)]),
        (HintMask, []),
        (HHCurveTo, [Integer(28), Integer(-29), Integer(-26), Integer(15), Integer(-31)]),
        (HVCurveTo, [Integer(-53), Integer(-43), Integer(-42), Integer(-68), Integer(-7),
                     Integer(1), Integer(-10), Integer(1), Integer(-6)]),
        (RMoveTo, [Integer(246), Integer(-479)]),
        (HintMask, []),
        (HHCurveTo, [Integer(-41), Integer(-58), Integer(-19), Integer(-14), Integer(-33)]),
        (HVCurveTo, [Integer(-24), Integer(-21), Integer(7), Integer(16), Integer(-15)]),
        (VVCurveTo, [Integer(-12), Integer(12), Integer(-8), Integer(15), Integer(26)]),
        (VHCurveTo, [Integer(30), Integer(11), Integer(39), Integer(87), Integer(34)]),
        (RRCurveTo, [Integer(21), Integer(8), Integer(36), Integer(12), Integer(35), Integer(10)]),
        (HintMask, []),
        (RMoveTo, [Integer(159), Integer(-196)]),
        (RLineTo, [Integer(-5), Integer(-5)]),
        (HHCurveTo, [Integer(-8), Integer(-8), Integer(-13), Integer(-9), Integer(-16)]),
        (HVCurveTo, [Integer(-22), Integer(-11), Integer(18), Integer(41)]),
        (VLineTo, [Integer(216)]),
        (VHCurveTo, [Integer(126), Integer(-50), Integer(48), Integer(-105), Integer(-102),
                     Integer(-74), Integer(-50), Integer(-76), Integer(-19)]),
        (HHCurveTo, [Integer(-26), Integer(3), Integer(16), Integer(-17), Integer(28)]),
        (HVCurveTo, [Integer(27), Integer(18), Integer(17), Integer(31), Integer(9)]),
        (RLineTo, [Integer(20), Integer(69)]),
        (HHCurveTo, [Integer(5), Integer(20), Integer(17), Integer(1), Integer(14)]),
        (HVCurveTo, [Integer(66), Integer(28), Integer(-24), Integer(-97)]),
        (VLineTo, [Integer(-27)]),
        (RRCurveTo, [Integer(-40), Integer(-9), Integer(-42), Integer(-13), Integer(-31),
                     Integer(-11)]),
        (VVCurveTo, [Integer(-135), Integer(-49), Integer(-31), Integer(-46), Integer(-57)]),
        (HintMask, []),
        (VHCurveTo, [Integer(-83), Integer(61), Integer(-44), Integer(73), Integer(59),
                     Integer(33), Integer(27), Integer(55), Integer(55)]),
        (HintMask, []),
        (HHCurveTo, [Integer(-47), Integer(8), Integer(31), Integer(-30), Integer(57)]),
        (HVCurveTo, [Integer(32), Integer(26), Integer(10), Integer(42), Integer(23)]),
    ]);
}
