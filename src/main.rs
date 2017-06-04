extern crate byteorder;
extern crate leb128;

mod wasm;

use self::wasm::{Compiler, Module, FunctionSignature, Type};
use self::wasm::module::sections::ExportEntry;
use self::wasm::module::ExternalKind;

fn main() {
	let mut module = Module::new();

	module.get_type_section().add_function_signature(
		FunctionSignature::new(vec!(Type::I64), Some(Type::F32))
	);
	module.get_function_section().add_function(0);
	module.get_export_section().add_export(ExportEntry{
		field_name: "exported_function".to_owned(),
		kind: ExternalKind::Function,
		index: 0,
	});

	Compiler::new().compile_to_file(&mut module, "hello_world.wasm");
}
