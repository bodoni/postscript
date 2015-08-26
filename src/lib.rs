//! Parser for PostScript fonts.

extern crate random;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error::new(::std::io::ErrorKind::Other, $message));
    );
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

mod band;

pub mod compact;
pub mod type2;
