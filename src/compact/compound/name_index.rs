index! {
    pub NameIndex
}

impl NameIndex {
    #[inline]
    pub fn get(&self, i: usize) -> Option<String> {
        self.0.get(i).and_then(|chunk| match chunk[0] {
            0 => None,
            _ => Some(String::from_utf8_lossy(chunk).into_owned()),
        })
    }
}
