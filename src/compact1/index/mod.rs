//! The indices.

use crate::compact1::{Offset, OffsetSize};
use crate::{Result, Tape, Value};

table! {
    @define
    #[doc = "An index."]
    pub Index {
        count       (u16         ), // count
        offset_size (OffsetSize  ), // offSize
        offsets     (Vec<Offset> ), // offset
        data        (Vec<Vec<u8>>), // data
    }
}

deref! { Index::data => [Vec<u8>] }

impl Value for Index {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
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
        for i in 0..(count as usize + 1) {
            let offset = tape.take_given::<Offset>(offset_size)?;
            if i == 0 && offset != Offset(1) || i > 0 && offset <= offsets[i - 1] {
                raise!("found a malformed index");
            }
            offsets.push(offset);
        }
        let mut data = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            let size = (offsets[i + 1].0 - offsets[i].0) as usize;
            data.push(tape.take_given(size)?);
        }
        Ok(Index {
            count: count,
            offset_size: offset_size,
            offsets: offsets,
            data: data,
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
        pub struct $structure(pub crate::compact1::index::Index);
        deref! { $structure::0 => crate::compact1::index::Index }
    );
    (@implement $structure:ident) => (
        impl typeface::Value for $structure {
            #[inline]
            fn read<T: typeface::Tape>(tape: &mut T) -> typeface::Result<Self> {
                Ok($structure(tape.take()?))
            }
        }
    );
}

mod char_strings;
mod dictionaries;
mod names;
mod strings;
mod subroutines;

pub use char_strings::CharStrings;
pub use dictionaries::Dictionaries;
pub use names::Names;
pub use strings::Strings;
pub use subroutines::Subroutines;
