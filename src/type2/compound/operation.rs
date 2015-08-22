use Result;
use band::{Band, Value};
use type2::primitive::Number;

pub type Operation = (Operator, Vec<Number>);

pub type Operations = Vec<Operation>;

impl Value for Operation {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let mut arguments = vec![];
        loop {
            match try!(band.peek::<u8>()) {
                0x1c | 0x20...0xff => arguments.push(try!(Value::read(band))),
                code => {
                    let code = if code == 0x0c {
                        try!(band.take::<u16>())
                    } else {
                        try!(band.take::<u8>()) as u16
                    };
                    let operator = match Operator::get(code) {
                        Some(operator) => operator,
                        _ => raise!("found an unknown operator"),
                    };
                    return Ok((operator, arguments));
                },
            }
        }
    }
}

impl Value for Operations {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let size = try!(band.count());
        let mut operations = vec![];
        while try!(band.position()) < size {
            let (operator, arguments) = try!(Value::read(band));
            operations.push((operator, arguments));
        }
        Ok(operations)
    }
}

macro_rules! operator {
    (pub $name:ident { $($code:pat => $variant:ident,)+ }) => (
        operator_define! { pub $name { $($variant,)+ } }
        operator_implement! { pub $name { $($code => $variant,)+ } }
    );
}

macro_rules! operator_define {
    (pub $name:ident { $($variant:ident,)* }) => (
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name { $($variant,)* }
    );
}

macro_rules! operator_implement {
    (pub $name:ident { $($code:pat => $variant:ident,)* }) => (impl $name {
        pub fn get(code: u16) -> Option<Self> {
            use self::$name::*;
            Some(match code {
                $($code => $variant,)+
                _ => return None,
            })
        }
    });
}

operator! {
    pub Operator {
        // 0x00 => Reserved,
        0x01 => hstem,
        // 0x02 => Reserved,
        0x03 => vstem,
        0x04 => vmoveto,
        0x05 => rlineto,
        0x06 => hlineto,
        0x07 => vlineto,
        0x08 => rrcurveto,
        // 0x09 => Reserved,
        0x0a => callsubr,
        0x0b => return_,
        0x0c => escape,
        // 0x0d => Reserved,
        0x0e => endchar,
        // 0x0f => Reserved,
        // 0x10 => Reserved,
        // 0x11 => Reserved,
        0x12 => hstemhm,
        0x13 => hintmask,
        0x14 => cntrmask,
        0x15 => rmoveto,
        0x16 => hmoveto,
        0x17 => vstemhm,
        0x18 => rcurveline,
        0x19 => rlinecurve,
        0x1a => vvcurveto,
        0x1b => hhcurveto,
        // 0x1c => shortint,
        0x1d => callgsubr,
        0x1e => vhcurveto,
        0x1f => hvcurveto,
        // 0x20...0xf6 => <numbers>,
        // 0xf7...0xfe => <numbers>,
        // 0xff => <number>,
        // 0x0c00 => Reserved,
        // 0x0c01 => Reserved,
        // 0x0c02 => Reserved,
        0x0c03 => and,
        0x0c04 => or,
        0x0c05 => not,
        // 0x0c06 => Reserved,
        // 0x0c07 => Reserved,
        // 0x0c08 => Reserved,
        0x0c09 => abs,
        0x0c0a => add,
        0x0c0b => sub,
        0x0c0c => div,
        // 0x0c0d => Reserved,
        0x0c0e => neg,
        0x0c0f => eq,
        // 0x0c10 => Reserved,
        // 0x0c11 => Reserved,
        0x0c12 => drop,
        // 0x0c13 => Reserved,
        0x0c14 => put,
        0x0c15 => get_,
        0x0c16 => ifelse,
        0x0c17 => random,
        0x0c18 => mul,
        // 0x0c19 => Reserved,
        0x0c1a => sqrt,
        0x0c1b => dup,
        0x0c1c => exch,
        0x0c1d => index,
        0x0c1e => roll,
        // 0x0c1f => Reserved,
        // 0x0c20 => Reserved,
        // 0x0c21 => Reserved,
        0x0c22 => hflex,
        0x0c23 => flex,
        0x0c24 => hflex1,
        0x0c25 => flex1,
        // 0x0c26...0x0cff => Reserved,
    }
}
