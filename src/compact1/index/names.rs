use crate::compact1::index::Index;
use crate::{Error, Result};

index! {
    #[doc = "A name index."]
    pub Names
}

impl TryFrom<Names> for Vec<String> {
    type Error = Error;

    fn try_from(names: Names) -> Result<Self> {
        let Names(Index { data, .. }) = names;
        let mut vector = Vec::with_capacity(data.len());
        for chunk in data {
            vector.push(match String::from_utf8(chunk) {
                Ok(string) => string,
                Err(chunk) => String::from_utf8_lossy(&chunk.into_bytes()).into_owned(),
            });
        }
        Ok(vector)
    }
}
