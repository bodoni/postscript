use std::collections::HashMap;
use std::io::Cursor;

use Result;
use band::{Band, Value};
use compact::compound::{Operand, Operator};

index! {
    pub DictionaryIndex
}

impl DictionaryIndex {
    pub fn get(&self, i: usize) -> Result<Option<HashMap<Operator, Vec<Operand>>>> {
        let chunk = match self.0.get(i) {
            Some(chunk) => chunk,
            _ => return Ok(None),
        };
        let size = chunk.len() as u64;
        let mut band = Cursor::new(chunk);
        let mut operations = HashMap::new();
        while try!(Band::position(&mut band)) < size {
            let (operator, operands) = try!(Value::read(&mut band));
            operations.insert(operator, operands);
        }
        Ok(Some(operations))
    }
}
