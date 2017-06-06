use wasm::FunctionSignature;
use super::{ModuleSection, SectionId};
use std::io::Write;
use leb128;

pub struct CodeSection {
	function_bodies: Vec<FunctionBody>
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
}

impl ModuleSection for CodeSection{
	fn get_id(&self) -> SectionId {
		SectionId::Id(10)
	}

	fn compile_payload<W: Write>(&self, out: &mut W) {
		leb128::write::unsigned(out, self.len() as u64).unwrap();//number of function bodies
		for function_body in &self.function_bodies{
			function_body.compile(out);
		}
	}
	fn is_empty(&self) -> bool {
		self.function_bodies.is_empty()
	}
}


pub struct FunctionBody{
	local_vars: Vec<LocalVar>
}
impl FunctionBody{
	pub fn compile<W: Write>(&self, out: &mut W){

	}
}

pub struct LocalVar{

}

