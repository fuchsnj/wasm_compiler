pub mod type_section;
pub mod function_section;
pub mod export_section;
pub mod code_section;

pub use self::type_section::TypeSection;
pub use self::function_section::FunctionSection;
pub use self::export_section::{ExportSection, ExportEntry};
pub use self::code_section::CodeSection;

use leb128;
use std::io::Write;
use write_sized_data;

pub trait ModuleSection{
	fn get_id(&self) -> SectionId;
	fn compile_payload<W: Write>(&self, out: &mut W);
	fn is_empty(&self) -> bool;
	fn compile<W: Write>(&self, out: &mut W){
		if !self.is_empty() {
			if let SectionId::Custom(_) = self.get_id() {
				panic!("Custom sections not supported yet");
			}

			leb128::write::unsigned(out, self.get_id().get_numeric_id() as u64).unwrap();//section id
			write_sized_data(out, |out|{
				self.compile_payload(out);
			});
		}
	}
}

pub enum SectionId{
	Id(u8),
	Custom(String)
}
impl SectionId{
	pub fn get_numeric_id(&self) -> u8{
		match *self{
			SectionId::Id(value) => value,
			SectionId::Custom(_) => 0
		}
	}
}
