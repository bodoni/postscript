use Result;
use band::{Band, Value};
use compact::primitive::{Integer, Real};

#[derive(Clone, Debug, PartialEq)]
pub struct Operation(pub Operator, pub Vec<Operand>);

pub type Operator = u16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    Integer(Integer),
    Real(Real),
}

impl Value for Operation {
    fn read<T: Band>(band: &mut T) -> Result<Operation> {
        let mut operands = vec![];
        loop {
            match try!(band.peek::<u8>()) {
                byte if byte <= 21 => {
                    let operator = if byte == 12 {
                        try!(u16::read(band))
                    } else {
                        try!(u8::read(band)) as u16
                    };
                    return Ok(Operation(operator, operands));
                },
                32...254 | 28 | 29 => operands.push(Operand::Integer(try!(Value::read(band)))),
                0x1e => operands.push(Operand::Real(try!(Value::read(band)))),
                _ => raise!("found a malformed operation"),
            };
        }
    }
}
