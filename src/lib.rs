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

pub mod compact;
pub mod type2;
