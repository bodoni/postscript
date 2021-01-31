//! The operations.

use Result;

/// An operand.
pub type Operand = f32;

/// An operation.
pub type Operation = (Operator, Vec<Operand>);

/// A collection of operations.
pub type Operations = Vec<Operation>;

macro_rules! operator {
    (pub $name:ident { $($code:pat => $variant:ident,)+ }) => (
        operator! { @define pub $name { $($variant,)+ } }
        operator! { @implement pub $name { $($code => $variant,)+ } }
    );
    (@define pub $name:ident { $($variant:ident,)* }) => (
        /// An operator.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name { $($variant,)* }
    );
    (@implement pub $name:ident { $($code:pat => $variant:ident,)* }) => (impl $name {
        #[doc(hidden)]
        pub fn from(code: u16) -> Result<Self> {
            use self::$name::*;
            Ok(match code {
                $($code => $variant,)+
                _ => raise!("found an unknown operator"),
            })
        }
    });
}

operator! {
    pub Operator {
        // 0x00 => Reserved,
        0x01 => HStem,
        // 0x02 => Reserved,
        0x03 => VStem,
        0x04 => VMoveTo,
        0x05 => RLineTo,
        0x06 => HLineTo,
        0x07 => VLineTo,
        0x08 => RRCurveTo,
        // 0x09 => Reserved,
        0x0a => CallSubr,
        0x0b => Return,
        // 0x0c => Escape,
        // 0x0d => Reserved,
        0x0e => EndChar,
        // 0x0f => Reserved,
        // 0x10 => Reserved,
        // 0x11 => Reserved,
        0x12 => HStemHM,
        0x13 => HintMask,
        0x14 => CntrMask,
        0x15 => RMoveTo,
        0x16 => HMoveTo,
        0x17 => VStemHM,
        0x18 => RCurveLine,
        0x19 => RLineCurve,
        0x1a => VVCurveTo,
        0x1b => HHCurveTo,
        // 0x1c => ShortInt,
        0x1d => CallGSubr,
        0x1e => VHCurveTo,
        0x1f => HVCurveTo,
        // 0x20..=0xf6 => <numbers>,
        // 0xf7..=0xfe => <numbers>,
        // 0xff => <number>,
        // 0x0c00 => Reserved,
        // 0x0c01 => Reserved,
        // 0x0c02 => Reserved,
        0x0c03 => And,
        0x0c04 => Or,
        0x0c05 => Not,
        // 0x0c06 => Reserved,
        // 0x0c07 => Reserved,
        // 0x0c08 => Reserved,
        0x0c09 => Abs,
        0x0c0a => Add,
        0x0c0b => Sub,
        0x0c0c => Div,
        // 0x0c0d => Reserved,
        0x0c0e => Neg,
        0x0c0f => Eq,
        // 0x0c10 => Reserved,
        // 0x0c11 => Reserved,
        0x0c12 => Drop,
        // 0x0c13 => Reserved,
        0x0c14 => Put,
        0x0c15 => Get,
        0x0c16 => IfElse,
        0x0c17 => Random,
        0x0c18 => Mul,
        // 0x0c19 => Reserved,
        0x0c1a => Sqrt,
        0x0c1b => Dup,
        0x0c1c => Exch,
        0x0c1d => Index,
        0x0c1e => Roll,
        // 0x0c1f => Reserved,
        // 0x0c20 => Reserved,
        // 0x0c21 => Reserved,
        0x0c22 => HFlex,
        0x0c23 => Flex,
        0x0c24 => HFlex1,
        0x0c25 => Flex1,
        // 0x0c26..=0x0cff => Reserved,
    }
}
