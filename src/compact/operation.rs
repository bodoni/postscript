//! The operations.

use std::collections::HashMap;

use {Result, Tape, Value};
use compact::number;

/// An operand.
pub type Operand = f32;

/// An operation.
pub type Operation = (Operator, Vec<Operand>);

/// A collection of operations.
#[derive(Clone, Debug, PartialEq)]
pub struct Operations(pub HashMap<Operator, Vec<Operand>>);

impl Value for Operation {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let mut operands = vec![];
        loop {
            match try!(tape.peek::<u8>()) {
                0x1c | 0x1d | 0x1e | 0x20...0xfe => operands.push(try!(number::read(tape))),
                code => {
                    let code = if code == 0x0c {
                        try!(tape.take::<u16>())
                    } else {
                        try!(tape.take::<u8>()) as u16
                    };
                    return Ok((try!(Operator::from(code)), operands));
                },
            }
        }
    }
}

impl Operations {
    /// Return the operands of an operation.
    #[inline]
    pub fn get(&self, operator: Operator) -> Option<&[Operand]> {
        match self.0.get(&operator) {
            Some(operands) => Some(&*operands),
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

deref! { Operations::0 => HashMap<Operator, Vec<Operand>> }

impl Value for Operations {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        use std::io::ErrorKind;

        let mut map = HashMap::new();
        loop {
            match tape.take() {
                Ok((operator, operands)) => {
                    map.insert(operator, operands);
                },
                Err(error) => {
                    if error.kind() == ErrorKind::UnexpectedEof {
                        return Ok(Operations(map));
                    } else {
                        return Err(error);
                    }
                },
            }
        }
    }
}

macro_rules! default(
    ([$($operand:expr),+]) => ({
        const OPERANDS: &'static [Operand] = &[$($operand as Operand),+];
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
                _ => raise!("found an unknown operator"),
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
        0x05 => FontBBox [0, 0, 0, 0],
        0x06 => BlueValues [],
        0x07 => OtherBlues [],
        0x08 => FamilyBlues [],
        0x09 => FamilyOtherBlues [],
        0x0a => StdHW [],
        0x0b => StdVW [],
        // 0x0c => Escape,
        0x0d => UniqueID [],
        0x0e => XUID [],
        0x0f => CharSet [0],
        0x10 => Encoding [0],
        0x11 => CharStrings [],
        0x12 => Private [],
        0x13 => Subrs [],
        0x14 => DefaultWidthX [0],
        0x15 => NominalWidthX [0],
        // 0x16...0x1b => Reserved,
        // 0x1c => ShortInt,
        // 0x1d => LongInt,
        // 0x1e => BCD,
        // 0x1f => Reserved,
        // 0x20...0xf6 => <numbers>,
        // 0xf7...0xfe => <numbers>,
        // 0xff => Reserved,
        0x0c00 => Copyright [],
        0x0c01 => IsFixedPitch [false as i32],
        0x0c02 => ItalicAngle [0],
        0x0c03 => UnderlinePosition [-100],
        0x0c04 => UnderlineThickness [50],
        0x0c05 => PaintType [0],
        0x0c06 => CharStringType [2],
        0x0c07 => FontMatrix [0.001, 0.0, 0.0, 0.001, 0.0, 0.0],
        0x0c08 => StrokeWidth [0],
        0x0c09 => BlueScale [0.039625],
        0x0c0a => BlueShift [7],
        0x0c0b => BlueFuzz [1],
        0x0c0c => StemSnapH [],
        0x0c0d => StemSnapV [],
        0x0c0e => ForceBold [false as i32],
        // 0x0c0f...0x0c10 => Reserved,
        0x0c11 => LanguageGroup [0],
        0x0c12 => ExpansionFactor [0.06],
        0x0c13 => InitialRandomSeed [0],
        0x0c14 => SyntheticBase [],
        0x0c15 => PostScript [],
        0x0c16 => BaseFontName [],
        0x0c17 => BaseFontBlend [],
        // 0x0c18...0x0c1d => Reserved,
        0x0c1e => ROS [],
        0x0c1f => CIDFontVersion [0],
        0x0c20 => CIDFontRevision [0],
        0x0c21 => CIDFontType [0],
        0x0c22 => CIDCount [8720],
        0x0c23 => UIDBase [],
        0x0c24 => FDArray [],
        0x0c25 => FDSelect [],
        0x0c26 => FontName [],
        // 0x0c27...0x0cff => Reserved,
    }
}
