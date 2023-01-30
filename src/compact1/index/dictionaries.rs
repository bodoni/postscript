use std::io::Cursor;

use crate::compact1::index::Index;
use crate::compact1::Operations;
use crate::{Error, Result, Tape};

index! {
    #[doc = "A dictionary index."]
    pub Dictionaries
}

impl TryFrom<Dictionaries> for Vec<Operations> {
    type Error = Error;

    fn try_from(dictionatires: Dictionaries) -> Result<Self> {
        let Dictionaries(Index { data, .. }) = dictionatires;
        let mut values = Vec::with_capacity(data.len());
        for chunk in data {
            values.push(Cursor::new(chunk).take()?);
        }
        Ok(values)
    }
}
