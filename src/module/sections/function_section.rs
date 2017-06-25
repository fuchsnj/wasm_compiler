use super::{ModuleSection, SectionId};
use std::io::Write;
use leb128;
use module::index_space::{IndexSpace, Handle};
use module::FunctionIndexSpace;

pub struct FunctionSection {
	function_indices: Vec<Handle>
}

impl FunctionSection {
	pub fn new() -> FunctionSection {
		FunctionSection {
			function_indices: vec!()
		}
	}
	pub fn len(&self) -> usize {
		self.function_indices.len()
	}
	pub fn add_function(&mut self, index: Handle) {
		self.function_indices.push(index);
	}
}

impl ModuleSection for FunctionSection{
	fn get_id(&self) -> SectionId {
		SectionId::Id(3)
	}

	fn compile_payload<W: Write>(&self, out: &mut W) {
		leb128::write::unsigned(out, self.len() as u64).unwrap();//number of function entries
		for function_index in &self.function_indices{
			leb128::write::unsigned(out, function_index.get_index()).unwrap();
		}
	}
	fn is_empty(&self) -> bool {
		self.function_indices.is_empty()
	}
}

