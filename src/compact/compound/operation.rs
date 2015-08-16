use Result;
use band::{Band, Value};
use compact::primitive::{Integer, Real};

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub operands: Vec<Operand>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    Integer(Integer),
    Real(Real),
}

impl Value for Operation {
    fn read<T: Band>(band: &mut T) -> Result<Operation> {
        let mut operands = vec![];
        loop {
            let mut code = try!(band.peek::<u8>()) as u16;
            let operator = match Operator::get(code) {
                Some(Operator::escape) => {
                    code = try!(band.peek::<u16>());
                    match Operator::get(code) {
                        Some(operator) => operator,
                        _ => raise!("found an unknown two-byte operator"),
                    }
                },
                Some(operator) => operator,
                _ => raise!("found an unknown one-byte operator"),
            };
            match operator {
                Operator::Integer => {
                    operands.push(Operand::Integer(try!(Value::read(band))));
                    continue;
                },
                Operator::Real => {
                    operands.push(Operand::Real(try!(Value::read(band))));
                    continue;
                },
                _ => {},
            }
            if code >> 8 == 0 {
                try!(band.take::<u8>());
            } else {
                try!(band.take::<u16>());
            }
            return Ok(Operation { operator: operator, operands: operands });
        }
    }
}

macro_rules! operator {
    (pub $name:ident { $($key:pat => $value:tt,)+ }) => (
        operator_define! { pub $name { $($value,)+ } }
        operator_implement! { pub $name { $($key => $value,)+ } }
    );
}

macro_rules! operator_define {
    ($name:ident [] [$($variant:ident,)*]) => (
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name { $($variant,)* }
    );
    ($name:ident [[$variant:ident], $($todo:tt)*] [$($done:tt)*]) => (
        operator_define!($name [$($todo)*] [$($done)*]);
    );
    ($name:ident [$variant:ident, $($todo:tt)*] [$($done:tt)*]) => (
        operator_define!($name [$($todo)*] [$($done)* $variant,]);
    );
    (pub $name:ident { $($todo:tt)* }) => (
        operator_define!($name [$($todo)*] []);
    );
}

macro_rules! operator_implement {
    ($name:ident [] [$($key:pat => $value:ident,)*]) => (
        impl $name {
            pub fn get(code: u16) -> Option<Self> {
                use self::$name::*;
                Some(match code {
                    $($key => $value,)+
                    _ => return None,
                })
            }
        }
    );
    ($name:ident [$key:pat => [$value:ident], $($todo:tt)*] [$($done:tt)*]) => (
        operator_implement!($name [$($todo)*] [$($done)* $key => $value,]);
    );
    ($name:ident [$key:pat => $value:ident, $($todo:tt)*] [$($done:tt)*]) => (
        operator_implement!($name [$($todo)*] [$($done)* $key => $value,]);
    );
    (pub $name:ident { $($todo:tt)* }) => (
        operator_implement!($name [$($todo)*] []);
    );
}

operator! {
    pub Operator {
        0x00 => version,
        0x01 => Notice,
        0x02 => FullName,
        0x03 => FamilyName,
        0x04 => Weight,
        0x05 => FontBBox,
        0x06 => BlueValues,
        0x07 => OtherBlues,
        0x08 => FamilyBlues,
        0x09 => FamilyOtherBlues,
        0x0a => StdHW,
        0x0b => StdVW,
        0x0c => escape,
        0x0d => UniqueID,
        0x0e => XUID,
        0x0f => charset,
        0x10 => Encoding,
        0x11 => CharStrings,
        0x12 => Private,
        0x13 => Subrs,
        0x14 => defaultWidthX,
        0x15 => nominalWidthX,
        // 0x16...0x1b => Reserved,
        0x1c...0x1d => Integer,
        0x1e => Real,
        // 0x1f => Reserved,
        0x20...0xfe => [Integer],
        // 0xff => Reserved,
        0x0c00 => Copyright,
        0x0c01 => isFixedPitch,
        0x0c02 => ItalicAngle,
        0x0c03 => UnderlinePosition,
        0x0c04 => UnderlineThickness,
        0x0c05 => PaintType,
        0x0c06 => CharstringType,
        0x0c07 => FontMatrix,
        0x0c08 => StrokeWidth,
        0x0c09 => BlueScale,
        0x0c0a => BlueShift,
        0x0c0b => BlueFuzz,
        0x0c0c => StemSnapH,
        0x0c0d => StemSnapV,
        0x0c0e => ForceBold,
        // 0x0c0f...0x0c10 => Reserved,
        0x0c11 => LanguageGroup,
        0x0c12 => ExpansionFactor,
        0x0c13 => initialRandomSeed,
        0x0c14 => SyntheticBase,
        0x0c15 => PostScript,
        0x0c16 => BaseFontName,
        0x0c17 => BaseFontBlend,
        // 0x0c18...0x0c1d => Reserved,
        0x0c1e => ROS,
        0x0c1f => CIDFontVersion,
        0x0c20 => IDFontRevision,
        0x0c21 => IDFontType,
        0x0c22 => IDCount,
        0x0c23 => IDBase,
        0x0c24 => DArray,
        0x0c25 => FDSelect,
        0x0c26 => FontName,
        // 0x0c27...0x0cff => Reserved,
    }
}
