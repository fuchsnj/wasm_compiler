use wasm::FunctionSignature;
use super::{ModuleSection, SectionId};
use std::io::Write;
use leb128;

pub struct CodeSection {
	function_bodies: Vec<()>
}

impl CodeSection {
	pub fn new() -> CodeSection {
		CodeSection {
			function_bodies: vec!()
		}
	}
	pub fn len(&self) -> usize {
		self.function_bodies.len()
	}
//	pub fn add_function(&mut self, index: u64) {
//		self.function_indices.push(index);
//	}
}

impl ModuleSection for CodeSection{
	fn get_id(&self) -> SectionId {
		SectionId::Id(10)
	}

	fn compile_payload<W: Write>(&self, out: &mut W) {
//		leb128::write::unsigned(out, self.len() as u64).unwrap();//number of function entries
//		for function_index in &self.function_indices{
//			leb128::write::unsigned(out, *function_index).unwrap();
//		}
	}
	fn is_empty(&self) -> bool {
		self.function_bodies.is_empty()
	}
}

