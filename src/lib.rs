//! Parser for PostScript fonts.

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error::new(::std::io::ErrorKind::Other, $message));
    );
);

mod band;

macro_rules! lookup {
    ($what:ident, $from:ty => $into:ty { $($key:expr => $value:expr,)+ }) => ({
        use std::collections::HashMap;
        use std::sync::{ONCE_INIT, Once};

        unsafe {
            static mut MAP: *const HashMap<$from, $into> = 0 as *const _;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                let mut map: HashMap<$from, $into> = HashMap::new();
                $(map.insert($key, $value);)+
                MAP = ::std::mem::transmute(Box::new(map));
            });
            (&*MAP).get(&$what).map(|&value| value)
        }
    });
}

pub mod compact;
