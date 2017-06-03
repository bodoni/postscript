use Result;
use compact1::index::Index;

index! {
    #[doc = "A name index."]
    pub Names
}

impl Names {
    #[doc(hidden)]
    pub fn into(self) -> Result<Vec<String>> {
        let Names(Index { data, .. }) = self;
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
