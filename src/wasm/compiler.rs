pub struct Compiler;

use std::io::{BufWriter, Write, Cursor};
use std::fs::File;
use super::module::{Module};
use byteorder::{LittleEndian, WriteBytesExt};
use wasm::module::sections::ModuleSection;

impl Compiler {
	pub fn new() -> Compiler {
		Compiler
	}

	pub fn compile<W: Write>(&self, module: &mut Module, out: W) {
		let mut buffer = BufWriter::new(out);
		module.compile(&mut buffer);
		buffer.flush();
	}

	pub fn compile_to_file(&self, module: &mut Module, filename: &str) {
		let file = File::create(filename).unwrap();
		self.compile(module, file);
	}
}