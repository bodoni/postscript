use std::io::Cursor;

use Result;
use band::{Band, Value};
use compact::compound::Operation;

index! {
    pub DictionaryIndex
}

impl DictionaryIndex {
    pub fn get(&self, i: usize) -> Result<Option<Vec<Operation>>> {
        let chunk = match self.0.get(i) {
            Some(chunk) => chunk,
            _ => return Ok(None),
        };
        let size = chunk.len() as u64;
        let mut band = Cursor::new(chunk);
        let mut operations = vec![];
        while try!(Band::position(&mut band)) < size {
            operations.push(try!(Value::read(&mut band)));
        }
        Ok(Some(operations))
    }
}
