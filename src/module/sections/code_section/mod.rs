pub mod bytecode;

use ty::ValueType;
use super::{ModuleSection, SectionId};
use std::io::Write;
use leb128;
use write_sized_data;
use self::bytecode::{AnyBytecode, Bytecode};

pub struct CodeSection {
	function_bodies: Vec<FunctionBody>
}

impl CodeSection {
	pub fn new() -> CodeSection {
		CodeSection {
			function_bodies: vec!()
		}
	}
	pub fn add_function_body(&mut self, bytecode: Vec<AnyBytecode>){
		self.function_bodies.push(FunctionBody::new(bytecode));
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
	local_vars: Vec<LocalVar>,
	bytecode: Vec<AnyBytecode>
}
impl FunctionBody{
	pub fn new(bytecode: Vec<AnyBytecode>) -> FunctionBody{
		FunctionBody{
			local_vars: Vec::new(),
			bytecode: bytecode
		}
	}
	pub fn compile<W: Write>(&self, out: &mut W){
		write_sized_data(out, |out|{
			leb128::write::unsigned(out, self.local_vars.len() as u64).unwrap();//number of local_vars
			for local_var in &self.local_vars{
				local_var.compile(out);
			}
			self.bytecode.compile(out).unwrap();
			out.write_all(&[0x0b]).unwrap();
		});
	}
}

pub struct LocalVar{
	value_type: ValueType
}
impl LocalVar{
	pub fn new(value_type: ValueType) -> LocalVar{
		LocalVar{
			value_type
		}
	}
	pub fn compile<W: Write>(&self, out: &mut W){
		//TODO: support more than 1 of each type per local variable entry
		leb128::write::unsigned(out, 1).unwrap();//number of vars of this type
		leb128::write::signed(out, self.value_type as i64).unwrap();//type of these variables
	}
}

