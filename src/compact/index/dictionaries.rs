use std::io::Cursor;

use Result;
use compact::index::Index;
use compact::operation::Operations;

index! {
    #[doc = "A dictionary index."]
    pub Dictionaries
}

impl Dictionaries {
    #[doc(hidden)]
    pub fn into(self) -> Result<Vec<Operations>> {
        let Dictionaries { index: Index { data, .. } } = self;
        let mut vector = Vec::with_capacity(data.len());
        for chunk in data {
            vector.push(read_value!(&mut Cursor::new(chunk)));
        }
        Ok(vector)
    }
}