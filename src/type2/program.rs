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
            ($from:expr, ...) => (flush!()[$from..].to_vec());
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
            macro_rules! done(() => (return Ok(Some((operator, flush!())))));
            match operator {
                HStem => done!(),
                VStem => done!(),
                VMoveTo => done!(),
                RLineTo => done!(),
                HLineTo => done!(),
                VLineTo => done!(),
                RRCurveTo => done!(),
                CallSubr => {},
                Return => {},
                Escape => {},
                EndChar => {},
                HStemHM => done!(),
                HintMask => {},
                CntrMask => {},
                RMoveTo => done!(),
                HMoveTo => done!(),
                VStemHM => done!(),
                RCurveLine => done!(),
                RLineCurve => done!(),
                VVCurveTo => done!(),
                HHCurveTo => done!(),
                CallGSubr => {},
                VHCurveTo => done!(),
                HVCurveTo => done!(),
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
            done!();
        }
    }
}