use std::io::Cursor;

use Result;
use band::Value;
use compact::compound::Operations;

index! {
    pub DictionaryIndex
}

impl DictionaryIndex {
    pub fn get(&self, i: usize) -> Result<Option<Operations>> {
        let chunk = match self.0.get(i) {
            Some(chunk) => chunk,
            _ => return Ok(None),
        };
        let mut band = Cursor::new(chunk);
        Ok(Some(try!(Value::read(&mut band))))
    }
}
