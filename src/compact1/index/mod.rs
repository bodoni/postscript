//! The indices.

use crate::compact1::{Offset, OffsetSize};
use crate::Result;

table! {
    @define
    /// An index.
    pub Index {
        count       (u16         ), // count
        offset_size (OffsetSize  ), // offSize
        offsets     (Vec<Offset> ), // offset
        data        (Vec<Vec<u8>>), // data
    }
}

dereference! { Index::data => [Vec<u8>] }

impl crate::value::Read for Index {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        let count = tape.take::<u16>()?;
        if count == 0 {
            return Ok(Index {
                count: 0,
                offset_size: 0,
                offsets: vec![],
                data: vec![],
            });
        }
        let offset_size = tape.take::<OffsetSize>()?;
        let mut offsets = Vec::with_capacity(count as usize + 1);
        for _ in 0..(count as usize + 1) {
            let offset = tape.take_given::<Offset>(offset_size)?;
            offsets.push(offset);
        }
        if offsets[0] != Offset(1) {
            raise!("found a malformed index");
        }
        let mut data = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            if offsets[i] > offsets[i + 1] {
                raise!("found a malformed index");
            }
            let size = (offsets[i + 1].0 - offsets[i].0) as usize;
            data.push(tape.take_given(size)?);
        }
        Ok(Index {
            count,
            offset_size,
            offsets,
            data,
        })
    }
}

macro_rules! index {
    ($(#[$attribute:meta])* pub $structure:ident) => (
        index! { @define $(#[$attribute])* pub $structure }
        index! { @implement $structure }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident) => (
        $(#[$attribute])*
        #[derive(Clone, Debug)]
        pub struct $structure(pub $crate::compact1::index::Index);
        dereference! { $structure::0 => $crate::compact1::index::Index }
    );
    (@implement $structure:ident) => (
        impl $crate::value::Read for $structure {
            #[inline]
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                Ok($structure(tape.take()?))
            }
        }
    );
}

mod character_strings;
mod dictionaries;
mod names;
mod strings;
mod subroutines;

pub use character_strings::CharacterStrings;
pub use dictionaries::Dictionaries;
pub use names::Names;
pub use strings::Strings;
pub use subroutines::Subroutines;
