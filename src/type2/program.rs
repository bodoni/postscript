use std::io::Cursor;
use std::mem;

use crate::{Result, Tape};
use crate::type2::{number, Operand, Operation, Operator};

/// A program.
pub struct Program<'l> {
    routine: Routine<'l>,
    global: &'l [Vec<u8>],
    local: &'l [Vec<u8>],
    stack: Vec<Operand>,
    stems: usize,
    width: Option<Operand>,
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
            width: None,
        }
    }

    /// Return the next operation.
    #[allow(unused_comparisons, unused_must_use)]
    pub fn next(&mut self) -> Result<Option<Operation>> {
        use crate::type2::Operator::*;

        if self.routine.done()? {
            return Ok(None);
        }

        macro_rules! pop(
            () => (match self.stack.pop() {
                Some(value) => value,
                _ => raise!("expected an operand"),
            });
            (bool) => (match self.stack.pop() {
                Some(value) => value != 0.0,
                _ => raise!("expected an operand"),
            });
            (i32) => (match self.stack.pop() {
                Some(value) if value as i32 as Operand == value => value as i32,
                _ => raise!("expected an operand of a different type"),
            });
        );
        macro_rules! push(
            ($operand:expr, bool) => ({
                let operand = $operand;
                self.stack.push(if operand { 1.0 } else { 0.0 });
            });
            ($operand:expr) => ({
                let operand = $operand;
                self.stack.push(operand);
            });
        );
        macro_rules! read(($index:expr) => ({
            let length = self.stack.len();
            if $index >= length {
                raise!("expected more operands");
            }
            self.stack[length - 1 - $index]
        }));

        let mut code;
        loop {
            code = self.routine.peek::<u8>()?;
            match code {
                0x1c | 0x20..=0xff => push!(self.routine.take_operand()?),
                _ => break,
            }
        }
        let operator = if code == 0x0c {
            Operator::from(self.routine.take::<u16>()?)?
        } else {
            Operator::from(self.routine.take::<u8>()? as u16)?
        };

        macro_rules! clear(
            (@reduce [$min:expr, $left:expr] []) => ({
                if $min > $left {
                    $min = $left;
                }
            });
            (@reduce [$min:expr, $left:expr] [equal($count:expr), $($tail:tt)*]) => ({
                if $left >= $count {
                    let left = $left - $count;
                    clear!(@reduce [$min, left] [$($tail)*]);
                }
            });
            (@reduce [$min:expr, $left:expr] [maybe_equal($count:expr), $($tail:tt)*]) => ({
                clear!(@reduce [$min, $left] [$($tail)*]);
                if $left >= $count {
                    clear!(@reduce [$min, $left - $count] [$($tail)*]);
                }
            });
            (@reduce [$min:expr, $left:expr] [modulo($count:expr), $($tail:tt)*]) => ({
                for i in 1..($left / $count + 1) {
                    let left = $left - i * $count;
                    clear!(@reduce [$min, left] [$($tail)*]);
                }
            });
            (@reduce [$min:expr, $left:expr] [maybe_modulo($count:expr), $($tail:tt)*]) => ({
                for i in 0..($left / $count + 1) {
                    let left = $left - i * $count;
                    clear!(@reduce [$min, left] [$($tail)*]);
                }
            });
            ($([$($predicate:ident($count:expr)),*]),+) => ({
                let length = self.stack.len();
                let mut min = !0;
                $(clear!(@reduce [min, length] [$($predicate($count),)*]);)+
                if min == !0 {
                    raise!("found malformed operands");
                }
                let mut stack = mem::replace(&mut self.stack, vec![]);
                let operands = stack.drain(min..).collect();
                if min > 0 && self.width.is_none() {
                    self.width = Some(stack[min - 1]);
                }
                return Ok(Some((operator, operands)));
            });
        );

        match operator {
            // Path-construction operators
            RMoveTo => clear!([equal(2)]),
            HMoveTo | VMoveTo => clear!([equal(1)]),
            RLineTo => clear!([modulo(2)]),
            HLineTo | VLineTo => clear!([equal(1), maybe_modulo(2)], [modulo(2)]),
            RRCurveTo => clear!([modulo(6)]),
            HHCurveTo | VVCurveTo => clear!([maybe_equal(1), modulo(4)]),
            HVCurveTo | VHCurveTo => clear!(
                [equal(4), maybe_modulo(8), maybe_equal(1)],
                [modulo(8), maybe_equal(1)]
            ),
            RCurveLine => clear!([modulo(6), equal(2)]),
            RLineCurve => clear!([modulo(2), equal(6)]),
            Flex => clear!([equal(13)]),
            Flex1 => clear!([equal(11)]),
            HFlex => clear!([equal(7)]),
            HFlex1 => clear!([equal(9)]),

            // Terminal operator
            EndChar => {
                while let Some(caller) = self.routine.caller.take() {
                    if !self.routine.done()? {
                        raise!("found trailing data after the end operator");
                    }
                    mem::replace(&mut self.routine, *caller);
                }
                let length = self.stack.len();
                if length > 0 && self.width.is_none() {
                    self.width = Some(self.stack[length - 1]);
                }
                return Ok(None);
            }

            // Hint operators
            HStem | VStem | HStemHM | VStemHM => {
                self.stems += self.stack.len() >> 1;
                clear!([equal(2), maybe_modulo(2)]);
            }
            HintMask | CntrMask => {
                self.stems += self.stack.len() >> 1;
                let _ = self.routine.take_given::<Vec<u8>>((self.stems + 7) >> 3)?;
                clear!([equal(0)]);
            }

            // Arithmetic operators
            Abs => push!(pop!().abs()),
            Add => push!(pop!() + pop!()),
            Sub => {
                let (right, left) = (pop!(), pop!());
                push!(left - right);
            }
            Div => {
                let (right, left) = (pop!(), pop!());
                push!(left / right);
            }
            Neg => push!(-pop!()),
            Random => unimplemented!(),
            Mul => push!(pop!() * pop!()),
            Sqrt => push!(pop!().sqrt()),
            Drop => mem::drop(pop!()),
            Exch => {
                let (right, left) = (pop!(), pop!());
                push!(right);
                push!(left);
            }
            Index => {
                let i = pop!(i32);
                push!(read!(if i >= 0 { i as usize } else { 0 }));
            }
            Roll => {
                let (shift, span) = (pop!(i32), pop!(i32));
                let length = self.stack.len();
                if span < 0 {
                    raise!("found an invalid operand");
                } else if span as usize > length {
                    raise!("expected more operands");
                } else if span > 0 {
                    let position = length - span as usize;
                    if shift > 0 {
                        for _ in 0..shift {
                            let operand = pop!();
                            self.stack.insert(position, operand);
                        }
                    } else if shift < 0 {
                        for _ in 0..(-shift) {
                            push!(self.stack.remove(position));
                        }
                    }
                }
            }
            Dup => push!(read!(0)),

            // Storage operators
            Put => unimplemented!(),
            Get => unimplemented!(),

            // Conditional operators
            And => {
                let (right, left) = (pop!(bool), pop!(bool));
                push!(left && right, bool);
            }
            Or => {
                let (right, left) = (pop!(bool), pop!(bool));
                push!(left || right, bool);
            }
            Not => push!(!pop!(bool), bool),
            Eq => {
                let (right, left) = (pop!(), pop!());
                push!(left == right, bool);
            }
            IfElse => {
                let (right, left, no, yes) = (pop!(), pop!(), pop!(), pop!());
                push!(if left <= right { yes } else { no });
            }

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
            }
            Return => {
                let caller = match self.routine.caller.take() {
                    Some(caller) => caller,
                    _ => raise!("found a return operator without a caller"),
                };
                mem::replace(&mut self.routine, *caller);
            }
        };
        self.next()
    }

    /// Return the width difference with respect to the nominal width.
    #[inline]
    pub fn width(&self) -> Option<Operand> {
        self.width
    }
}

impl<'l> Routine<'l> {
    #[inline]
    fn new(code: &'l [u8]) -> Routine<'l> {
        Routine {
            tape: Cursor::new(code),
            size: code.len(),
            caller: None,
        }
    }

    #[inline]
    fn done(&mut self) -> Result<bool> {
        Ok(Tape::position(&mut self.tape)? == self.size as u64)
    }

    #[inline]
    fn take_operand(&mut self) -> Result<Operand> {
        number::read(&mut self.tape)
    }
}

deref! { Routine<'l>::tape => Cursor<&'l [u8]> }

#[inline]
fn bias(count: usize) -> i32 {
    if count < 1240 {
        107
    } else if count < 33900 {
        1131
    } else {
        32768
    }
}
