use std::io::Cursor;

use Result;
use band::Band;
use type2::compound::{Operation, Operator};
use type2::primitive::Number;

pub struct Machine<'l> {
    global: &'l [Vec<u8>],
    local: &'l [Vec<u8>],
    stack: Vec<Number>,
}

pub struct Routine<'l> {
    machine: &'l mut Machine<'l>,
    band: Cursor<&'l [u8]>,
    size: usize,
}

impl<'l> Machine<'l> {
    #[inline]
    pub fn new(global: &'l [Vec<u8>], local: &'l [Vec<u8>]) -> Machine<'l> {
        Machine { global: global, local: local, stack: vec![] }
    }

    #[inline]
    pub fn execute(&'l mut self, code: &'l [u8]) -> Routine<'l> {
        Routine { machine: self, band: Cursor::new(code), size: code.len() }
    }
}

impl<'l> Routine<'l> {
    pub fn next(&mut self) -> Result<Option<Operation>> {
        use std::mem;
        use type2::compound::Operator::*;

        let stack = &mut self.machine.stack;
        let band = &mut self.band;

        macro_rules! done(() => (try!(Band::position(band)) == self.size as u64));
        macro_rules! flush(() => (mem::replace(stack, vec![])));

        if done!() {
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
            match operator {
                CallSubr => {},
                Return => {},
                EndChar => {
                    if !done!() {
                        raise!("found trailing data after the end operator");
                    }
                },
                HintMask => {},
                CntrMask => {},
                CallGSubr => {},
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
                _ => {},
            }
            return Ok(Some((operator, flush!())));
        }
    }
}
