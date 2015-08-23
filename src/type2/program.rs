use std::io::Cursor;

use band::{Band, Value};
use type2::compound::{Operation, Operator};
use type2::primitive::Number;

pub struct Program<'l> {
    band: Cursor<&'l [u8]>,
    local: &'l [&'l [u8]],
    global: &'l [&'l [u8]],
    stack: Vec<Number>,
}

impl<'l> Program<'l> {
    #[inline]
    pub fn new(code: &'l [u8], local: &'l [&'l [u8]], global: &'l [&'l [u8]]) -> Program<'l> {
        Program { band: Cursor::new(code), global: global, local: local, stack: vec![] }
    }
}

impl<'l> Iterator for Program<'l> {
    type Item = Operation;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
