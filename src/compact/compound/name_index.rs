use compact::compound::index;

index! {
    pub NameIndex
}

impl NameIndex {
    #[inline]
    pub fn get(&self, i: usize) -> Option<String> {
        index::string(self, i)
    }
}
