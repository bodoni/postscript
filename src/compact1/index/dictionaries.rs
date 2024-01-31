use std::io::Cursor;

use crate::compact1::Operations;
use crate::tape::Read;
use crate::{Error, Result};

index! {
    #[doc = "A dictionary index."]
    pub Dictionaries
}

impl Dictionaries {
    /// Return the operations at a specific position.
    #[inline]
    pub fn get(&self, index: usize) -> Result<Operations> {
        debug_assert!(index < self.len());
        Cursor::new(&self[index]).take()
    }
}

impl TryFrom<&Dictionaries> for Vec<Operations> {
    type Error = Error;

    fn try_from(dictionatires: &Dictionaries) -> Result<Self> {
        let mut values = Vec::with_capacity(dictionatires.len());
        for index in 0..dictionatires.len() {
            values.push(Dictionaries::get(dictionatires, index)?);
        }
        Ok(values)
    }
}
