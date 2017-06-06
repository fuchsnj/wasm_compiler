mod type_section;
mod function_section;
mod export_section;

pub use self::type_section::TypeSection;
pub use self::function_section::FunctionSection;
pub use self::export_section::{ExportSection, ExportEntry};

use leb128;
use std::io::{Write, Cursor};

pub trait ModuleSection{
	fn get_id(&self) -> SectionId;
	fn compile_payload<W: Write>(&self, out: &mut W);
	fn is_empty(&self) -> bool;
	fn compile<W: Write>(&self, out: &mut W){
		if !self.is_empty() {
			if let SectionId::Custom(_) = self.get_id() {
				panic!("Custom sections not supported");
			}
			let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec!());
			self.compile_payload(&mut cursor);
			let inner_buffer = cursor.into_inner();
			let payload_len = inner_buffer.len();

			leb128::write::unsigned(out, self.get_id().get_numeric_id() as u64).unwrap();//section id
			leb128::write::unsigned(out, payload_len as u64).unwrap();//payload size + name
			out.write_all(&inner_buffer).unwrap();
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
