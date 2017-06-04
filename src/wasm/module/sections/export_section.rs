use wasm::FunctionSignature;
use super::{ModuleSection, SectionId};
use std::io::Write;
use leb128;
use wasm::module::ExternalKind;
//use super::super::

pub struct ExportSection {
	export_entries: Vec<ExportEntry>
//	function_indices: Vec<u64>
}

impl ExportSection {
	pub fn new() -> ExportSection {
		ExportSection {
			export_entries: vec!()
		}
	}
	pub fn len(&self) -> usize {
		self.export_entries.len()
	}
	pub fn add_export(&mut self, entry: ExportEntry) {
		self.export_entries.push(entry);
	}
}

impl ModuleSection for ExportSection{
	fn get_id(&self) -> SectionId {
		SectionId::Id(7)
	}

	fn compile_payload<W: Write>(&self, out: &mut W) {
		leb128::write::unsigned(out, self.len() as u64).unwrap();//number of entries
		for entry in &self.export_entries{
			entry.compile(out);
		}
	}
	fn is_empty(&self) -> bool {
		self.export_entries.is_empty()
	}
}

pub struct ExportEntry{
	pub field_name: String,
	pub kind: ExternalKind,
	pub index: u64 //into corresponding index space
}
impl ExportEntry{
	fn compile<W: Write>(&self, out: &mut W){
		leb128::write::unsigned(out, self.field_name.len() as u64).unwrap();//length of name
		out.write_all(self.field_name.as_bytes()).unwrap();
		out.write_all(&[self.kind as u8]).unwrap();
		leb128::write::unsigned(out, self.index).unwrap();
	}
}

