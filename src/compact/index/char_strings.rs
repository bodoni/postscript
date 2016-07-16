use {Result, Tape, Walue};

index! {
    @define
    #[doc = "A char-string index."]
    pub CharStrings
}

impl Walue<i32> for CharStrings {
    fn read<T: Tape>(tape: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharStrings { index: read_value!(tape) },
            _ => raise!("found an unknown char-string format"),
        })
    }
}
