//! The indices.

use {Result, Tape, Value};
use compact::{Offset, OffsetSize};

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
        let count = try!(tape.take::<u16>());
        if count == 0 {
            return Ok(Index { count: 0, offset_size: 0, offsets: vec![], data: vec![] });
        }
        let offset_size = try!(tape.take::<OffsetSize>());
        let mut offsets = Vec::with_capacity(count as usize + 1);
        for i in 0..(count as usize + 1) {
            let offset = read_walue!(tape, offset_size, Offset);
            if i == 0 && offset != Offset(1) || i > 0 && offset <= offsets[i - 1] {
                raise!("found a malformed index");
            }
            offsets.push(offset);
        }
        let mut data = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            let size = (u32::from(offsets[i + 1]) - u32::from(offsets[i])) as usize;
            data.push(read_walue!(tape, size));
        }
        Ok(Index { count: count, offset_size: offset_size, offsets: offsets, data: data })
    }
}

macro_rules! index {
    ($(#[$attribute:meta])* pub $structure:ident) => (
        index! { @define $(#[$attribute])* pub $structure }
        index! { @implement $structure }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure(pub ::compact::index::Index);
        deref! { $structure::0 => ::compact::index::Index }
    );
    (@implement $structure:ident) => (
        impl ::tape::Value for $structure {
            #[inline]
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                Ok($structure(read_value!(tape)))
            }
        }
    );
}

mod char_strings;
mod dictionaries;
mod names;
mod strings;
mod subroutines;

pub use self::char_strings::CharStrings;
pub use self::dictionaries::Dictionaries;
pub use self::names::Names;
pub use self::strings::Strings;
pub use self::subroutines::Subroutines;
