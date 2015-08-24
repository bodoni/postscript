use std::io::Cursor;
use std::mem;

use Result;
use band::Band;
use type2::compound::{Operation, Operator};
use type2::primitive::Number;

#[allow(dead_code)]
pub struct Program<'l> {
    band: Cursor<&'l [u8]>,
    size: usize,
    global: &'l [Vec<u8>],
    local: &'l [Vec<u8>],
    stack: Vec<Number>,
}

impl<'l> Program<'l> {
    #[inline]
    pub fn new(code: &'l [u8], global: &'l [Vec<u8>], local: &'l [Vec<u8>]) -> Program<'l> {
        Program {
            band: Cursor::new(code),
            size: code.len(),
            global: global,
            local: local,
            stack: vec![],
        }
    }

    pub fn next(&mut self) -> Result<Option<Operation>> {
        use type2::compound::Operator::*;

        let band = &mut self.band;
        let stack = &mut self.stack;

        macro_rules! flush(
            () => (mem::replace(stack, vec![]));
            (all) => ({
                if stack.len() == 0 {
                    raise!("expected more arguments");
                }
                flush!()
            });
            (all, even) => ({
                let count = stack.len();
                if count == 0 || count % 2 != 0 {
                    raise!("expected an even number of arguments");
                }
                flush!()
            });
            (from $from:expr) => ({
                if stack.len() <= $from {
                    raise!("expected more arguments");
                }
                flush!()[$from..].to_vec()
            });
            (until $until:expr) => ({
                if stack.len() < $until  {
                    raise!("expected more arguments");
                }
                flush!()[..$until].to_vec()
            });
        );

        if try!(Band::position(band)) == self.size as u64 {
            return Ok(None);
        }
        loop {
            let code = match try!(band.peek::<u8>()) {
                0x1c | 0x20...0xff => {
                    stack.push(try!(band.take()));
                    continue;
                },
                code if code == 0x0c => try!(band.take::<u16>()),
                _ => try!(band.take::<u8>()) as u16,
            };
            let operator = match Operator::get(code) {
                Some(operator) => operator,
                _ => raise!("found an unknown operator ({:#x})", code),
            };
            macro_rules! done(
                ($arguments:expr) => (return Ok(Some((operator, $arguments))));
            );
            match operator {
                HStem | VStem | HStemHM | VStemHM => {
                    if stack.len() % 2 == 0 {
                        done!(flush!(all));
                    } else {
                        done!(flush!(from 1));
                    }
                },
                VMoveTo | HMoveTo => done!(flush!(until 1)),
                RLineTo => done!(flush!(all, even)),
                HLineTo => {},
                VLineTo => {},
                RRCurveTo => {},
                CallSubr => {},
                Return => {},
                Escape => {},
                EndChar => {},
                HintMask => {},
                CntrMask => {},
                RMoveTo => {},
                RCurveLine => {},
                RLineCurve => {},
                VVCurveTo => {},
                HHCurveTo => {},
                CallGSubr => {},
                VHCurveTo => {},
                HVCurveTo => {},
                And => {},
                Or => {},
                Not => {},
                Abs => {},
                Add => {},
                Sub => {},
                Div => {},
                Neg => {},
                Eq => {},
                Drop => {},
                Put => {},
                Get => {},
                IfElse => {},
                Random => {},
                Mul => {},
                Sqrt => {},
                Dup => {},
                Exch => {},
                Index => {},
                Roll => {},
                HFlex => {},
                Flex => {},
                HFlex1 => {},
                Flex1 => {},
            }
            done!(flush!(all));
        }
    }
}
