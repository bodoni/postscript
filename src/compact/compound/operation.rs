use Result;
use band::{Band, Value};
use compact::primitive::{Integer, Real};

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub operands: Vec<Operand>,
}

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
                        try!(band.take::<u16>())
                    } else {
                        try!(band.take::<u8>()) as u16
                    };
                    return Ok(Operation { operator: operator, operands: operands });
                },
                32...254 | 28 | 29 => operands.push(Operand::Integer(try!(Value::read(band)))),
                0x1e => operands.push(Operand::Real(try!(Value::read(band)))),
                _ => raise!("found a malformed operation"),
            };
        }
    }
}
