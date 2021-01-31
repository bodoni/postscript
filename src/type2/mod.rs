//! The [Type 2 char-string format][1].
//!
//! [1]: http://wwwimages.adobe.com/content/dam/Adobe/en/devnet/font/pdfs/5177.Type2.pdf

mod number;
mod operation;
mod program;

pub use operation::{Operand, Operation, Operations, Operator};
pub use program::Program;
