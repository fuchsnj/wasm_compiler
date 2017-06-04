pub mod sections;

mod external_kind;

pub use self::external_kind::ExternalKind;

use self::sections::{ModuleSection, TypeSection, FunctionSection, ExportSection};
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};

pub struct Module {
	type_section: TypeSection,//section 1
	//import section - 2
	function_section: FunctionSection,//section 3
	//table section - 4
	//memory section - 5
	//globals section - 6
	export_section: ExportSection,
	//exports section - 7
	//start section - 8
	//elements section - 9
	//code section - 10
	//data section - 11
}

impl Module {
	pub fn new() -> Module {
		Module {
			type_section: TypeSection::new(),
			function_section: FunctionSection::new(),
			export_section: ExportSection::new()
		}
	}

	pub fn get_type_section(&mut self) -> &mut TypeSection {
		&mut self.type_section
	}

	pub fn get_function_section(&mut self) -> &mut FunctionSection {
		&mut self.function_section
	}

	pub fn get_export_section(&mut self) -> &mut ExportSection{
		&mut self.export_section
	}

	pub fn compile<W: Write>(&mut self, out: &mut W) {
		out.write_all(b"\0asm");//magic number
		out.write_u32::<LittleEndian>(1);//version

		self.get_type_section().compile(out);//1
		self.get_function_section().compile(out);//3
		self.get_export_section().compile(out);//7

	}
}