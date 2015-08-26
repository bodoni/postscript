#![allow(dead_code)]

use random::{self, Source};
use std::io::Cursor;
use std::mem;

use Result;
use band::{Band, ValueExt};
use type2::compound::{Operation, Operator};
use type2::primitive::Number;

pub struct Program<'l> {
    routine: Routine<'l>,
    global: &'l [Vec<u8>],
    local: &'l [Vec<u8>],
    stack: Vec<Number>,
    stems: usize,
    source: Box<Source>,
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
            source: Box::new(random::default()),
        }
    }

    pub fn next(&mut self) -> Result<Option<Operation>> {
        use type2::compound::Operator::*;

        if try!(self.routine.done()) {
            return Ok(None);
        }

        macro_rules! pop(() => (match self.stack.pop() {
            Some(value) => value,
            _ => raise!("expected an argument in the stack"),
        }));
        macro_rules! push(($argument:expr) => ({
            let argument = $argument;
            self.stack.push(argument);
        }));
        macro_rules! top(() => ({
            let count = self.stack.len();
            if count == 0 {
                raise!("expected an argument in the stack");
            }
            self.stack[count - 1]
        }));

        loop {
            let code = match try!(self.routine.peek::<u8>()) {
                0x1c | 0x20...0xff => {
                    push!(try!(self.routine.take()));
                    continue;
                },
                code if code == 0x0c => try!(self.routine.take::<u16>()),
                _ => try!(self.routine.take::<u8>()) as u16,
            };
            let operator = match Operator::get(code) {
                Some(operator) => operator,
                _ => raise!("found an unknown operator ({:#x})", code),
            };
            macro_rules! flush(
                () => (Ok(Some((operator, mem::replace(&mut self.stack, vec![])))));
            );
            match operator {
                /// Path-construction operators
                RMoveTo | HMoveTo | VMoveTo | RLineTo | HLineTo | VLineTo |
                RRCurveTo | HHCurveTo | HVCurveTo | VHCurveTo | VVCurveTo |
                RCurveLine | RLineCurve | Flex | Flex1 | HFlex | HFlex1 => return flush!(),

                /// Terminal operator
                EndChar => return self.unwind(),

                /// Hint operators
                HStem | VStem | HStemHM | VStemHM => {
                    self.stems += self.stack.len() >> 1;
                    return flush!();
                },
                HintMask | CntrMask => {
                    self.stems += self.stack.len() >> 1;
                    let _: Vec<u8> = try!(ValueExt::read(&mut *self.routine,
                                                         (self.stems + 7) >> 3));
                    return flush!();
                },

                /// Arithmetic operators
                Abs => push!(pop!().abs()),
                Add => push!(pop!() + pop!()),
                Sub => {
                    let (right, left) = (pop!(), pop!());
                    push!(left - right);
                },
                Div => {
                    let (right, left) = (pop!(), pop!());
                    push!(left / right);
                },
                Neg => push!(-pop!()),
                Random => push!(Number::Real(self.source.read_f64() as f32)),
                Mul => push!(pop!() * pop!()),
                Sqrt => push!(pop!().sqrt()),
                Drop => {
                    pop!();
                },
                Exch => {
                    let (right, left) = (pop!(), pop!());
                    push!(right);
                    push!(left);
                },
                Index => unimplemented!(),
                Roll => unimplemented!(),
                Dup => push!(top!()),

                // Storage operators
                Put => unimplemented!(),
                Get => unimplemented!(),

                /// Conditional operators
                And => push!(pop!().and(pop!())),
                Or => push!(pop!().or(pop!())),
                Not => push!(!pop!()),
                Eq => push!(pop!().equal(pop!())),
                IfElse => {
                    let (right, left, no, yes) = (pop!(), pop!(), pop!(), pop!());
                    push!(if left <= right { yes } else { no });
                },

                /// Subroutine operators
                CallSubr | CallGSubr => return self.call(operator),
                Return => return self.recall(),
            };
            return self.next();
        }
    }

    pub fn seed(&mut self, seed: [u64; 2]) {
        self.source = Box::new(random::default().seed(seed));
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
            if !try!(self.routine.done()) {
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

deref! { Routine<'l>::band => Cursor<&'l [u8]> }

#[inline]
fn bias(count: usize) -> i32 {
    if count < 1240 { 107 } else if count < 33900 { 1131 } else { 32768 }
}
