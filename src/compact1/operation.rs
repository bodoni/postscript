//! The operations.

use crate::compact1::number::Number;
use crate::{Result, Tape, Value};

/// An operand.
pub type Operand = Number;

/// An operation.
#[derive(Clone, Debug)]
pub struct Operation(pub Operator, pub Vec<Operand>);

/// A collection of operations.
#[derive(Clone, Debug, Default)]
pub struct Operations(pub Vec<Operation>);

dereference! { Operations::0 => Vec<Operation> }

impl Value for Operation {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let mut operands = vec![];
        loop {
            match tape.peek::<u8>()? {
                0x1c | 0x1d | 0x1e | 0x20..=0xfe => operands.push(tape.take()?),
                code => {
                    let code = if code == 0x0c {
                        tape.take::<u16>()?
                    } else {
                        tape.take::<u8>()? as u16
                    };
                    return Ok(Self(Operator::from(code)?, operands));
                }
            }
        }
    }
}

impl Operations {
    /// Return the operands of an operation.
    #[inline]
    pub fn get(&self, operator: Operator) -> Option<&[Operand]> {
        match self.0.iter().position(|operation| operation.0 == operator) {
            Some(index) => Some(&self.0[index].1),
            _ => operator.default(),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn get_single(&self, operator: Operator) -> Option<Operand> {
        self.get(operator).and_then(|operands| {
            if operands.len() > 0 {
                Some(operands[0])
            } else {
                None
            }
        })
    }

    #[doc(hidden)]
    #[inline]
    pub fn get_double(&self, operator: Operator) -> Option<(Operand, Operand)> {
        self.get(operator).and_then(|operands| {
            if operands.len() > 1 {
                Some((operands[0], operands[1]))
            } else {
                None
            }
        })
    }
}

impl Value for Operations {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        use std::io::ErrorKind;

        let mut records = vec![];
        loop {
            match tape.take() {
                Ok(operation) => {
                    records.push(operation);
                }
                Err(error) => {
                    if error.kind() == ErrorKind::UnexpectedEof {
                        return Ok(Operations(records));
                    } else {
                        return Err(error);
                    }
                }
            }
        }
    }
}

macro_rules! default(
    ([$($operand:expr),+ $(,)?]) => ({
        const OPERANDS: &'static [Operand] = &[$($operand),+];
        Some(OPERANDS)
    });
    ([]) => (None);
);

macro_rules! operator {
    (pub $name:ident { $($code:pat => $variant:ident $default:tt,)+ }) => (
        operator! { @define pub $name { $($variant,)+ } }
        operator! { @implement pub $name { $($code => $variant $default,)+ } }
    );
    (@define pub $name:ident { $($variant:ident,)* }) => (
        /// An operator.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum $name { $($variant,)* }
    );
    (@implement pub $name:ident { $($code:pat => $variant:ident $default:tt,)* }) => (impl $name {
        #[doc(hidden)]
        pub fn from(code: u16) -> Result<Self> {
            use self::$name::*;

            Ok(match code {
                $($code => $variant,)+
                code => raise!("found an unsupported operator ({})", code),
            })
        }

        /// Return the default operands.
        pub fn default(&self) -> Option<&'static [Operand]> {
            use self::$name::*;

            match *self {
                $($variant => default!($default),)+
            }
        }
    });
}

operator! {
    pub Operator {
        0x00 => Version [],
        0x01 => Notice [],
        0x02 => FullName [],
        0x03 => FamilyName [],
        0x04 => Weight [],
        0x05 => FontBBox [
            Number::Integer(0),
            Number::Integer(0),
            Number::Integer(0),
            Number::Integer(0),
        ],
        0x06 => BlueValues [],
        0x07 => OtherBlues [],
        0x08 => FamilyBlues [],
        0x09 => FamilyOtherBlues [],
        0x0a => StdHW [],
        0x0b => StdVW [],
        // 0x0c => Escape,
        0x0d => UniqueID [],
        0x0e => XUID [],
        0x0f => CharSet [Number::Integer(0)],
        0x10 => Encoding [Number::Integer(0)],
        0x11 => CharStrings [],
        0x12 => Private [],
        0x13 => Subrs [],
        0x14 => DefaultWidthX [Number::Integer(0)],
        0x15 => NominalWidthX [Number::Integer(0)],
        // 0x16..=0x1b => Reserved,
        // 0x1c => ShortInt,
        // 0x1d => LongInt,
        // 0x1e => BCD,
        // 0x1f => Reserved,
        // 0x20..=0xf6 => <numbers>,
        // 0xf7..=0xfe => <numbers>,
        // 0xff => Reserved,
        0x0c00 => Copyright [],
        0x0c01 => IsFixedPitch [Number::Integer(false as i32)],
        0x0c02 => ItalicAngle [Number::Integer(0)],
        0x0c03 => UnderlinePosition [Number::Integer(-100)],
        0x0c04 => UnderlineThickness [Number::Integer(50)],
        0x0c05 => PaintType [Number::Integer(0)],
        0x0c06 => CharStringType [Number::Integer(2)],
        0x0c07 => FontMatrix [
            Number::Real(0.001),
            Number::Real(0.0),
            Number::Real(0.0),
            Number::Real(0.001),
            Number::Real(0.0),
            Number::Real(0.0),
        ],
        0x0c08 => StrokeWidth [Number::Integer(0)],
        0x0c09 => BlueScale [Number::Real(0.039625)],
        0x0c0a => BlueShift [Number::Integer(7)],
        0x0c0b => BlueFuzz [Number::Integer(1)],
        0x0c0c => StemSnapH [],
        0x0c0d => StemSnapV [],
        0x0c0e => ForceBold [Number::Integer(false as i32)],
        // 0x0c0f..=0x0c10 => Reserved,
        0x0c11 => LanguageGroup [Number::Integer(0)],
        0x0c12 => ExpansionFactor [Number::Real(0.06)],
        0x0c13 => InitialRandomSeed [Number::Integer(0)],
        0x0c14 => SyntheticBase [],
        0x0c15 => PostScript [],
        0x0c16 => BaseFontName [],
        0x0c17 => BaseFontBlend [],
        // 0x0c18..=0x0c1d => Reserved,
        0x0c1e => ROS [],
        0x0c1f => CIDFontVersion [Number::Integer(0)],
        0x0c20 => CIDFontRevision [Number::Integer(0)],
        0x0c21 => CIDFontType [Number::Integer(0)],
        0x0c22 => CIDCount [Number::Integer(8720)],
        0x0c23 => UIDBase [],
        0x0c24 => FDArray [],
        0x0c25 => FDSelect [],
        0x0c26 => FontName [],
        // 0x0c27..=0x0cff => Reserved,
    }
}
