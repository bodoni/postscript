//! The [Type 2 Charstring Format][1].
//!
//! [1]: https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf

mod number;
mod operation;
mod program;

pub use operation::{Operand, Operation, Operations, Operator};
pub use program::Program;
