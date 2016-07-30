//! The [Type 2 char-string format][1].
//!
//! [1]: http://partners.adobe.com/public/developer/en/font/5177.Type2.pdf

mod number;
mod program;

pub mod operation;

pub use self::operation::Operations;
pub use self::program::Program;
