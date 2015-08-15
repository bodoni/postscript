index! {
    pub NameIndex
}

impl NameIndex {
    pub fn strings(&self) -> Vec<String> {
        let index = &self.0;
        let count = index.count as usize;
        let mut strings = Vec::with_capacity(count);
        let mut from = 0;
        for i in 0..count {
            let until = index.offset[i + 1] as usize;
            let mut slice = &index.data[from..until];
            if let Some(j) = slice.iter().position(|&byte| byte == 0) {
                slice = &slice[0..j];
            }
            if slice.is_empty() {
                continue;
            }
            let string = String::from_utf8_lossy(slice);
            strings.push(string.into_owned());
            from = until;
        }
        strings
    }
}
