pub struct Compiler;

use std::io::{BufWriter, Write};
use std::fs::File;
use super::module::Module;

impl Compiler {
	pub fn new() -> Compiler {
		Compiler
	}

	pub fn compile<W: Write>(&self, module: &mut Module, out: W) {
		let mut buffer = BufWriter::new(out);
		module.compile(&mut buffer);
		buffer.flush().unwrap();
	}

	pub fn compile_to_file(&self, module: &mut Module, filename: &str) {
		let file = File::create(filename).unwrap();
		self.compile(module, file);
	}
}