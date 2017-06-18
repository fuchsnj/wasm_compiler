use super::Bytecode;
use std::io::Write;
use std::io;
use leb128;

const CALL: u8 = 0x10;
const CALL_INDIRECT: u8 = 0x11;

const RESERVED_VALUE: u32 = 0;

pub struct Call{
	function_index: u32
}
impl Bytecode for Call{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[CALL])?;
		leb128::write::unsigned(out, self.function_index as u64);
		Ok(())
	}
}

pub struct CallIndirect{
	type_index: u32
}
impl Bytecode for CallIndirect{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[CALL_INDIRECT])?;
		leb128::write::unsigned(out, self.type_index as u64)?;
		leb128::write::unsigned(out, RESERVED_VALUE as u64)?;
		Ok(())
	}
}