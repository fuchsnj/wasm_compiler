use super::Bytecode;
use std::io::Write;
use std::io;
use leb128;
use byteorder::{WriteBytesExt, LittleEndian};

const I32_CONST: u8 = 0x41;
const I64_CONST: u8 = 0x42;
const F32_CONST: u8 = 0x43;
const F64_CONST: u8 = 0x44;

pub struct I32Constant {
	value: i32
}

impl Bytecode for I32Constant {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_CONST])?;
		leb128::write::signed(out, self.value as i64)?;
		Ok(())
	}
}

pub struct I64Constant {
	value: i64
}

impl Bytecode for I64Constant {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_CONST])?;
		leb128::write::signed(out, self.value)?;
		Ok(())
	}
}

pub struct F32Constant {
	value: u32
}

impl Bytecode for F32Constant {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_CONST])?;
		out.write_u32::<LittleEndian>(self.value).unwrap();
		Ok(())
	}
}

pub struct F64Constant {
	value: u64
}

impl Bytecode for F64Constant {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_CONST])?;
		out.write_u64::<LittleEndian>(self.value).unwrap();
		Ok(())
	}
}