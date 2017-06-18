use super::Bytecode;
use std::io::Write;
use std::io;
use leb128;

const GET_LOCAL: u8 = 0x20;
const SET_LOCAL: u8 = 0x21;
const TEE_LOCAL: u8 = 0x22;
const GET_GLOBAL: u8 = 0x23;
const SET_GLOBAL: u8 = 0x24;

pub struct GetLocal {
	local_index: u32
}

impl Bytecode for GetLocal {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[GET_LOCAL])?;
		leb128::write::unsigned(out, self.local_index as u64)?;
		Ok(())
	}
}

pub struct SetLocal {
	local_index: u32
}

impl Bytecode for SetLocal {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[SET_LOCAL])?;
		leb128::write::unsigned(out, self.local_index as u64)?;
		Ok(())
	}
}

pub struct TeeLocal {
	local_index: u32
}

impl Bytecode for TeeLocal {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[TEE_LOCAL])?;
		leb128::write::unsigned(out, self.local_index as u64)?;
		Ok(())
	}
}

pub struct GetGlobal {
	local_index: u32
}

impl Bytecode for GetGlobal {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[GET_GLOBAL])?;
		leb128::write::unsigned(out, self.local_index as u64)?;
		Ok(())
	}
}

pub struct SetGlobal {
	local_index: u32
}

impl Bytecode for SetGlobal {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[SET_GLOBAL])?;
		leb128::write::unsigned(out, self.local_index as u64)?;
		Ok(())
	}
}