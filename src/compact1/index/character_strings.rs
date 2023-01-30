use crate::{Result, Tape, Walue};

index! {
    @define
    #[doc = "A character-string index."]
    pub CharacterStrings
}

impl Walue<'static> for CharacterStrings {
    type Parameter = i32;

    fn read<T: Tape>(tape: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharacterStrings(tape.take()?),
            format => raise!(
                "found an unsupported format of character strings ({})",
                format,
            ),
        })
    }
}
