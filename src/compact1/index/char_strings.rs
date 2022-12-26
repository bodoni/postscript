use crate::{Result, Tape, Walue};

index! {
    @define
    #[doc = "A char-string index."]
    pub CharStrings
}

impl Walue<'static> for CharStrings {
    type Parameter = i32;

    fn read<T: Tape>(tape: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharStrings(tape.take()?),
            _ => raise!("found an unknown format of the char strings"),
        })
    }
}
