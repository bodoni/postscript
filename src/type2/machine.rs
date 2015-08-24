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
    pub fn start(&'l mut self, code: &'l [u8]) -> Routine<'l> {
        Routine { machine: self, band: Cursor::new(code), size: code.len() }
    }

    pub fn execute(&'l mut self, code: &'l [u8]) -> Result<Vec<Operation>> {
        let mut routine = self.start(code);
        let mut operations = vec![];
        while let Some(operation) = try!(routine.next()) {
            operations.push(operation);
        }
        Ok(operations)
    }
}

impl<'l> Routine<'l> {
    pub fn next(&mut self) -> Result<Option<Operation>> {
        use std::mem;
        use type2::compound::Operator::*;

        macro_rules! done(
            () => (try!(Band::position(&mut self.band)) == self.size as u64);
        );
        macro_rules! dump(
            () => (mem::replace(&mut self.machine.stack, vec![]));
        );
        macro_rules! peek(
            ($kind:ty) => (try!(self.band.peek::<$kind>()));
        );
        macro_rules! push(
            ($argument:expr) => (self.machine.stack.push($argument));
        );
        macro_rules! take(
            () => (try!(self.band.take()));
            ($kind:ty) => (try!(self.band.take::<$kind>()));
        );

        if done!() {
            return Ok(None);
        }
        loop {
            let code = match peek!(u8) {
                0x1c | 0x20...0xff => {
                    push!(take!());
                    continue;
                },
                code if code == 0x0c => take!(u16),
                _ => take!(u8) as u16,
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
            return Ok(Some((operator, dump!())));
        }
    }
}
