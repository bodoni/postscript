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

        macro_rules! flush(() => (mem::replace(&mut self.stack, vec![])));
        macro_rules! pop(
            () => (match self.stack.pop() {
                Some(value) => value,
                _ => raise!("expected an argument"),
            });
            ($kind:ident) => (match self.stack.pop() {
                Some(Number::$kind(value)) => value,
                _ => raise!("expected an argument of a different type"),
            });
        );
        macro_rules! push(($argument:expr) => ({
            let argument = $argument;
            self.stack.push(argument);
        }));
        macro_rules! read(($index:expr) => ({
            let count = self.stack.len();
            if $index >= count {
                raise!("expected more arguments");
            }
            self.stack[count - 1 - $index]
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
            match operator {
                /// Path-construction operators
                RMoveTo | HMoveTo | VMoveTo | RLineTo | HLineTo | VLineTo |
                RRCurveTo | HHCurveTo | HVCurveTo | VHCurveTo | VVCurveTo |
                RCurveLine | RLineCurve | Flex | Flex1 | HFlex | HFlex1 => {
                    return Ok(Some((operator, flush!())));
                },

                /// Terminal operator
                EndChar => {
                    while let Some(caller) = self.routine.caller.take() {
                        if !try!(self.routine.done()) {
                            raise!("found trailing data after the end operator");
                        }
                        mem::replace(&mut self.routine, *caller);
                    }
                    return Ok(None);
                },

                /// Hint operators
                HStem | VStem | HStemHM | VStemHM => {
                    self.stems += self.stack.len() >> 1;
                    return Ok(Some((operator, flush!())));
                },
                HintMask | CntrMask => {
                    self.stems += self.stack.len() >> 1;
                    let _: Vec<u8> = try!(ValueExt::read(&mut *self.routine,
                                                         (self.stems + 7) >> 3));
                    return Ok(Some((operator, flush!())));
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
                Random => unimplemented!(),
                Mul => push!(pop!() * pop!()),
                Sqrt => push!(pop!().sqrt()),
                Drop => { pop!(); },
                Exch => {
                    let (right, left) = (pop!(), pop!());
                    push!(right);
                    push!(left);
                },
                Index => {
                    let i = pop!(Integer);
                    push!(read!(if i >= 0 { i as usize } else { 0 }));
                },
                Roll => unimplemented!(),
                Dup => push!(read!(0)),

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
                CallSubr | CallGSubr => {
                    let address = pop!(Integer);
                    let mut routine = {
                        let subroutines = if operator == CallSubr {
                            &self.local
                        } else {
                            &self.global
                        };
                        let count = subroutines.len();
                        let i = address + bias(count);
                        if i < 0 || i as usize >= count {
                            raise!("failed to find a subroutine");
                        }
                        Routine::new(&subroutines[i as usize])
                    };
                    mem::swap(&mut self.routine, &mut routine);
                    self.routine.caller = Some(Box::new(routine));
                },
                Return => {
                    let caller = match self.routine.caller.take() {
                        Some(caller) => caller,
                        _ => raise!("found a return operator without a caller"),
                    };
                    mem::replace(&mut self.routine, *caller);
                },
            };
            return self.next();
        }
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
