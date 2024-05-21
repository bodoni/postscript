use crate::Result;

index! {
    @define
    /// A character-string index.
    pub CharacterStrings
}

impl crate::walue::Read<'static> for CharacterStrings {
    type Parameter = i32;

    fn read<T: crate::tape::Read>(tape: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharacterStrings(tape.take()?),
            format => raise!("found an unknown format of character strings ({format})"),
        })
    }
}
