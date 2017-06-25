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
pub use self::ty::{Type, ValueType, BlockType};
pub use self::function::{FunctionSignature};

use std::io::{Cursor, Write};

fn write_sized_data<F, W: Write>(out: &mut W, inner_func: F) where F: FnOnce(&mut Cursor<Vec<u8>>){
	let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec!());
	inner_func(&mut cursor);//write data to buffer
	let inner_buffer = cursor.into_inner();
	let data_len = inner_buffer.len();
	leb128::write::unsigned(out, data_len as u64).unwrap();//data size
	out.write_all(&inner_buffer).unwrap();//write data after size
}