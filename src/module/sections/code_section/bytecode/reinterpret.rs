use super::Bytecode;
use std::io::Write;
use std::io;

const I32_REINTERPRET_F32: u8 = 0xbc;
const I32_REINTERPRET_F64: u8 = 0xbd;
const F32_REINTERPRET_I32: u8 = 0xbe;
const F64_REINTERPRET_I64: u8 = 0xbf;

pub struct I32ReinterpretF32;
impl Bytecode for I32ReinterpretF32{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_REINTERPRET_F32])
	}
}

pub struct I32ReinterpretF64;
impl Bytecode for I32ReinterpretF64{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_REINTERPRET_F64])
	}
}

pub struct F32ReinterpretI32;
impl Bytecode for F32ReinterpretI32{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_REINTERPRET_I32])
	}
}

pub struct F64ReinterpretI64;
impl Bytecode for F64ReinterpretI64{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_REINTERPRET_I64])
	}
}