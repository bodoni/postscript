index! {
    pub NameIndex
}

impl NameIndex {
    pub fn strings(&self) -> Vec<String> {
        let count = self.count as usize;
        let mut strings = Vec::with_capacity(count);
        for i in 0..count {
            let from = self.offset[i] as usize - 1;
            if self.data[from] == 0 {
                continue;
            }
            let until = self.offset[i + 1] as usize - 1;
            strings.push(String::from_utf8_lossy(&self.data[from..until]).into_owned());
        }
        strings
    }
}
