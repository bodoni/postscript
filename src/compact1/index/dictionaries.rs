use std::io::Cursor;

use {Result, Tape};
use compact1::index::Index;
use compact1::Operations;

index! {
    #[doc = "A dictionary index."]
    pub Dictionaries
}

impl Dictionaries {
    #[doc(hidden)]
    pub fn into(self) -> Result<Vec<Operations>> {
        let Dictionaries(Index { data, .. }) = self;
        let mut values = Vec::with_capacity(data.len());
        for chunk in data {
            values.push(Cursor::new(chunk).take()?);
        }
        Ok(values)
    }
}