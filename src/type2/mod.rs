//! The Type 2 charstring format.

pub mod compound;
pub mod primitive;

mod machine;

pub use self::machine::{Machine, Program};
