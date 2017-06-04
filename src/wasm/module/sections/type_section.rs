use wasm::FunctionSignature;
use super::{ModuleSection, SectionId};
use std::io::Write;
use leb128;

pub struct TypeSection {
	type_list: Vec<FunctionSignature>
}

impl TypeSection {
	pub fn new() -> TypeSection {
		TypeSection {
			type_list: vec!()
		}
	}
	pub fn len(&self) -> usize {
		self.type_list.len()
	}
	pub fn add_function_signature(&mut self, func: FunctionSignature) {
		self.type_list.push(func);
	}
}

impl ModuleSection for TypeSection{
	fn get_id(&self) -> SectionId {
		SectionId::Id(1)
	}

	fn compile_payload<W: Write>(&self, out: &mut W) {
		leb128::write::unsigned(out, self.len() as u64);//number of type entries
		for type_entry in &self.type_list{
			type_entry.compile(out);
		}
	}
	fn is_empty(&self) -> bool {
		self.type_list.is_empty()
	}
}

