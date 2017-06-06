extern crate byteorder;
extern crate leb128;

pub mod module;
mod compiler;
mod ty;
mod function;

#[cfg(test)]
mod test;

pub use self::compiler::Compiler;
pub use self::module::Module;
pub use self::ty::{Type, ValueType};
pub use self::function::{FunctionSignature};