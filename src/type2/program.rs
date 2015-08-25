#![allow(dead_code)]

use std::io::Cursor;
use std::mem;
use std::ops::{Deref, DerefMut};

use Result;
use band::Band;
use type2::compound::{Operation, Operator};
use type2::primitive::Number;

pub struct Program<'l> {
    routine: Routine<'l>,
    global: &'l [Vec<u8>],
    local: &'l [Vec<u8>],
    stack: Vec<Number>,
    stems: usize,
}

struct Routine<'l> {
    band: Cursor<&'l [u8]>,
    size: usize,
    caller: Option<Box<Routine<'l>>>,
}

impl<'l> Program<'l> {
    #[inline]
    pub fn new(code: &'l [u8], global: &'l [Vec<u8>], local: &'l [Vec<u8>]) -> Program<'l> {
        Program {
            routine: Routine::new(code),
            global: global,
            local: local,
            stack: vec![],
            stems: 0,
        }
    }

    pub fn next(&mut self) -> Result<Option<Operation>> {
        use type2::compound::Operator::*;
        if try!(self.routine.done()) {
            return Ok(None);
        }
        loop {
            let code = match try!(self.routine.peek::<u8>()) {
                0x1c | 0x20...0xff => {
                    self.stack.push(try!(self.routine.take()));
                    continue;
                },
                code if code == 0x0c => try!(self.routine.take::<u16>()),
                _ => try!(self.routine.take::<u8>()) as u16,
            };
            let operator = match Operator::get(code) {
                Some(operator) => operator,
                _ => raise!("found an unknown operator ({:#x})", code),
            };
            match operator {
                HStem | VStem | HStemHM | VStemHM => {
                    self.stems += self.stack.len() >> 1;
                },
                CallSubr | CallGSubr => return self.call(operator),
                Return => return self.recall(),
                EndChar => return self.unwind(),
                HintMask => {
                },
                CntrMask => {
                },
                // And => {},
                // Or => {},
                // Not => {},
                // Abs => {},
                // Add => {},
                // Sub => {},
                // Div => {},
                // Neg => {},
                // Eq => {},
                // Drop => {},
                // Put => {},
                // Get => {},
                // IfElse => {},
                // Random => {},
                // Mul => {},
                // Sqrt => {},
                // Dup => {},
                // Exch => {},
                // Index => {},
                // Roll => {},
                _ => {},
            };
            return Ok(Some((operator, mem::replace(&mut self.stack, vec![]))));
        }
    }

    fn call(&mut self, operator: Operator) -> Result<Option<Operation>> {
        let address = match self.stack.pop() {
            Some(Number::Integer(address)) => address,
            _ => raise!("expected an argument"),
        };
        let mut routine = if let Operator::CallGSubr = operator {
            let count = self.global.len();
            let i = address + bias(count);
            if i < 0 || i as usize >= count {
                raise!("failed to find a global subroutine");
            }
            Routine::new(&self.global[i as usize])
        } else {
            let count = self.local.len();
            let i = address + bias(count);
            if i < 0 || i as usize >= count {
                raise!("failed to find a local subroutine");
            }
            Routine::new(&self.local[i as usize])
        };
        mem::swap(&mut self.routine, &mut routine);
        self.routine.caller = Some(Box::new(routine));
        self.next()
    }

    fn recall(&mut self) -> Result<Option<Operation>> {
        let caller = match self.routine.caller.take() {
            Some(caller) => caller,
            _ => raise!("found a return operator without a caller"),
        };
        mem::replace(&mut self.routine, *caller);
        self.next()
    }

    fn unwind(&mut self) -> Result<Option<Operation>> {
        while let Some(caller) = self.routine.caller.take() {
            if try!(self.routine.done()) {
                raise!("found trailing data after the end operator");
            }
            mem::replace(&mut self.routine, *caller);
        }
        Ok(None)
    }
}

impl<'l> Routine<'l> {
    #[inline]
    fn new(code: &'l [u8]) -> Routine<'l> {
        Routine { band: Cursor::new(code), size: code.len(), caller: None }
    }

    #[inline]
    fn done(&mut self) -> Result<bool> {
        Ok(try!(Band::position(&mut self.band)) == self.size as u64)
    }
}

impl<'l> Deref for Routine<'l> {
    type Target = Cursor<&'l [u8]>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.band
    }
}

impl<'l> DerefMut for Routine<'l> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.band
    }
}

#[inline]
fn bias(count: usize) -> i32 {
    if count < 1240 { 107 } else if count < 33900 { 1131 } else { 32768 }
}
