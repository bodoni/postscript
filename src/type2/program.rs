use std::io::Cursor;
use std::mem;

use {Result, Tape};
use type2::operation::{Operation, Operator};

/// A program.
pub struct Program<'l> {
    routine: Routine<'l>,
    global: &'l [Vec<u8>],
    local: &'l [Vec<u8>],
    stack: Vec<f32>,
    stems: usize,
}

struct Routine<'l> {
    tape: Cursor<&'l [u8]>,
    size: usize,
    caller: Option<Box<Routine<'l>>>,
}

impl<'l> Program<'l> {
    /// Create a program.
    #[inline]
    pub fn new(code: &'l [u8], global: &'l [Vec<u8>], local: &'l [Vec<u8>]) -> Self {
        Program {
            routine: Routine::new(code),
            global: global,
            local: local,
            stack: vec![],
            stems: 0,
        }
    }

    /// Return the next operation.
    pub fn next(&mut self) -> Result<Option<Operation>> {
        use type2::operation::Operator::*;

        if try!(self.routine.done()) {
            return Ok(None);
        }

        macro_rules! clear(
            () => (mem::replace(&mut self.stack, vec![]));
        );
        macro_rules! pop(
            () => (match self.stack.pop() {
                Some(value) => value,
                _ => raise!("expected an argument"),
            });
            (bool) => (match self.stack.pop() {
                Some(value) => value != 0.0,
                _ => raise!("expected an argument"),
            });
            (i32) => (match self.stack.pop() {
                Some(value) if value as i32 as f32 == value => value as i32,
                _ => raise!("expected an argument of a different type"),
            });
        );
        macro_rules! push(
            ($argument:expr) => ({
                let argument = $argument;
                self.stack.push(argument);
            });
        );
        macro_rules! read(($index:expr) => ({
            let length = self.stack.len();
            if $index >= length {
                raise!("expected more arguments");
            }
            self.stack[length - 1 - $index]
        }));

        let mut code;
        loop {
            code = try!(self.routine.peek::<u8>());
            match code {
                0x1c | 0x20...0xff => push!(try!(self.routine.take_number())),
                _ => break,
            }
        }
        let operator = if code == 0x0c {
            try!(Operator::from(try!(self.routine.take::<u16>())))
        } else {
            try!(Operator::from(try!(self.routine.take::<u8>()) as u16))
        };
        match operator {
            // Path-construction operators
            RMoveTo | HMoveTo | VMoveTo | RLineTo | HLineTo | VLineTo |
            RRCurveTo | HHCurveTo | HVCurveTo | VHCurveTo | VVCurveTo |
            RCurveLine | RLineCurve | Flex | Flex1 | HFlex | HFlex1 => {
                return Ok(Some((operator, clear!())));
            },

            // Terminal operator
            EndChar => {
                while let Some(caller) = self.routine.caller.take() {
                    if !try!(self.routine.done()) {
                        raise!("found trailing data after the end operator");
                    }
                    mem::replace(&mut self.routine, *caller);
                }
                return Ok(None);
            },

            // Hint operators
            HStem | VStem | HStemHM | VStemHM => {
                self.stems += self.stack.len() >> 1;
                return Ok(Some((operator, clear!())));
            },
            HintMask | CntrMask => {
                self.stems += self.stack.len() >> 1;
                let _: Vec<u8> = read_walue!(&mut *self.routine, (self.stems + 7) >> 3);
                return Ok(Some((operator, clear!())));
            },

            // Arithmetic operators
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
            Drop => {
                pop!();
            },
            Exch => {
                let (right, left) = (pop!(), pop!());
                push!(right);
                push!(left);
            },
            Index => {
                let i = pop!(i32);
                push!(read!(if i >= 0 { i as usize } else { 0 }));
            },
            Roll => {
                let (shift, span) = (pop!(i32), pop!(i32));
                let length = self.stack.len();
                if span < 0 {
                    raise!("found an invalid argument");
                } else if span as usize > length {
                    raise!("expected more arguments");
                } else if span > 0 {
                    let position = length - span as usize;
                    if shift > 0 {
                        for _ in 0..shift {
                            let argument = pop!();
                            self.stack.insert(position, argument);
                        }
                    } else if shift < 0 {
                        for _ in 0..(-shift) {
                            push!(self.stack.remove(position));
                        }
                    }
                }
            },
            Dup => push!(read!(0)),

            // Storage operators
            Put => unimplemented!(),
            Get => unimplemented!(),

            // Conditional operators
            And => {
                let (right, left) = (pop!(bool), pop!(bool));
                push!(if left && right { 1.0 } else { 0.0 })
            },
            Or => {
                let (right, left) = (pop!(bool), pop!(bool));
                push!(if left || right { 1.0 } else { 0.0 })
            },
            Not => push!(if pop!(bool) { 0.0 } else { 1.0 }),
            Eq => {
                let (right, left) = (pop!(), pop!());
                push!(if left == right { 1.0 } else { 0.0 })
            },
            IfElse => {
                let (right, left, no, yes) = (pop!(), pop!(), pop!(), pop!());
                push!(if left <= right { yes } else { no });
            },

            // Subroutine operators
            CallSubr | CallGSubr => {
                let address = pop!(i32);
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
        self.next()
    }
}

impl<'l> Routine<'l> {
    #[inline]
    fn new(code: &'l [u8]) -> Routine<'l> {
        Routine { tape: Cursor::new(code), size: code.len(), caller: None }
    }

    #[inline]
    fn done(&mut self) -> Result<bool> {
        Ok(try!(Tape::position(&mut self.tape)) == self.size as u64)
    }

    fn take_number(&mut self) -> Result<f32> {
        const FIXED_SCALING: f32 = 1f32 / (1 << 16) as f32;
        macro_rules! read(($kind:ident) => (try!(self.tape.take::<$kind>())));
        let first = read!(u8);
        Ok(match first {
            0x20...0xf6 => (first as i32 - 139) as f32,
            0xf7...0xfa => ((first as i32 - 247) * 256 + read!(u8) as i32 + 108) as f32,
            0xfb...0xfe => (-(first as i32 - 251) * 256 - read!(u8) as i32 - 108) as f32,
            0x1c => read!(u16) as i16 as i32 as f32,
            0xff => FIXED_SCALING * (read!(u32) as f32),
            _ => raise!("found a malformed number"),
        })
    }
}

deref! { Routine<'l>::tape => Cursor<&'l [u8]> }

#[inline]
fn bias(count: usize) -> i32 {
    if count < 1240 { 107 } else if count < 33900 { 1131 } else { 32768 }
}

#[cfg(test)]
mod tests {
    use super::Routine;

    #[test]
    fn routine_take_number() {
        let code = vec![0xff, 0x00, 0x01, 0x04, 0x5a];
        let mut routine = Routine::new(&code);
        assert_eq!(format!("{:.3}", routine.take_number().unwrap()), "1.017");
    }
}
