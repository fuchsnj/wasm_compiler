pub mod sections;
pub mod index_space;

mod external_kind;

pub use self::external_kind::ExternalKind;

use self::sections::{ModuleSection, TypeSection, FunctionSection, ExportSection, CodeSection};
use function::FunctionSignature;
use module::sections::export_section::ExportEntry;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};
use self::index_space::{IndexSpace, IndexSection};
use self::sections::code_section::bytecode::AnyBytecode;
use ValueType;


pub struct Module {
	type_section: TypeSection,
	//section 1
	//import section - 2
	function_section: FunctionSection,
	//section 3
	//table section - 4
	//memory section - 5
	//globals section - 6
	export_section: ExportSection,
	//section 7
	//start section - 8
	//elements section - 9
	code_section: CodeSection,
	//section 10
	//data section - 11

	function_index: IndexSpace<FunctionIndexSpace>
}

impl Module {
	pub fn new() -> Module {
		Module {
			type_section: TypeSection::new(),
			function_section: FunctionSection::new(),
			export_section: ExportSection::new(),
			code_section: CodeSection::new(),
			function_index: IndexSpace::new()
		}
	}

	pub fn compile<W: Write>(&mut self, out: &mut W) {
		out.write_all(b"\0asm").unwrap();//magic number
		out.write_u32::<LittleEndian>(1).unwrap();//version

		self.type_section.compile(out);//1
		self.function_section.compile(out);//3
		self.export_section.compile(out);//7
		self.code_section.compile(out);//10
	}

	pub fn build_function(&mut self, params: &[ValueType], return_type: Option<ValueType>) ->
	FunctionBuilder {
		FunctionBuilder{
			module: self,
			params: params.to_vec(),
			return_type,
			export: None,
			bytecode: vec!()
		}
	}
}

pub struct FunctionBuilder<'a>{
	module: &'a mut Module,
	params: Vec<ValueType>,
	return_type: Option<ValueType>,
	export: Option<String>,
	bytecode: Vec<AnyBytecode>
}
impl <'a> FunctionBuilder<'a>{
	pub fn export(mut self, name: &str) -> Self{
		self.export = Some(name.to_owned()); self
	}
	pub fn code(mut self, bytecode: AnyBytecode) -> Self{
		self.bytecode.push(bytecode); self
	}
	pub fn code_list(mut self, bytecode: Vec<AnyBytecode>) -> Self{
		for code in bytecode{
			self.bytecode.push(code);
		}
		self
	}
	pub fn build(mut self){
		let signature = FunctionSignature::new(self.params, self.return_type);
		self.module.type_section.add_function_signature(signature);
		let index = self.module.function_index.allocate_index(FunctionIndexSpace::DeclaredSection);
		self.module.function_section.add_function(index);
		self.module.code_section.add_function_body(self.bytecode);
		if let Some(export_name) = self.export {
			self.module.export_section.add_export(ExportEntry {
				field_name: export_name,
				kind: ExternalKind::Function,
				index: 0, //TODO: calculate real index
			})
		}
	}
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum FunctionIndexSpace {
	ImportSection,
	DeclaredSection
}

impl IndexSection for FunctionIndexSpace {
	fn get_sections() -> Vec<Self> {
		vec![FunctionIndexSpace::ImportSection, FunctionIndexSpace::DeclaredSection]
	}
}