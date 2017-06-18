use super::Bytecode;
use std::io::Write;
use std::io;
use leb128;
use byteorder::{WriteBytesExt, LittleEndian};

const I32_EQZ: u8 = 0x45;
const I32_EQ: u8 = 0x46;
const I32_NE: u8 = 0x47;
const I32_LT_S: u8 = 0x48;
const I32_LT_U: u8 = 0x49;
const I32_GT_S: u8 = 0x4a;
const I32_GT_U: u8 = 0x4b;
const I32_LE_S: u8 = 0x4c;
const I32_LE_U: u8 = 0x4d;
const I32_GE_S: u8 = 0x4e;
const I32_GE_U: u8 = 0x4f;

const I64_EQZ: u8 = 0x50;
const I64_EQ: u8 = 0x51;
const I64_NE: u8 = 0x52;
const I64_LT_S: u8 = 0x53;
const I64_LT_U: u8 = 0x54;
const I64_GT_S: u8 = 0x55;
const I64_GT_U: u8 = 0x56;
const I64_LE_S: u8 = 0x57;
const I64_LE_U: u8 = 0x58;
const I64_GE_S: u8 = 0x59;
const I64_GE_U: u8 = 0x5a;

const F32_EQ: u8 = 0x5b;
const F32_NE: u8 = 0x5c;
const F32_LT: u8 = 0x5d;
const F32_GT: u8 = 0x5e;
const F32_LE: u8 = 0x5f;
const F32_GE: u8 = 0x60;

const F64_EQ: u8 = 0x61;
const F64_NE: u8 = 0x62;
const F64_LT: u8 = 0x63;
const F64_GT: u8 = 0x64;
const F64_LE: u8 = 0x65;
const F64_GE: u8 = 0x66;

pub struct I32EqualToZero;
impl Bytecode for I32EqualToZero{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_EQZ])
	}
}

pub struct I32Equal;
impl Bytecode for I32Equal{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_EQ])
	}
}

pub struct I32NotEqual;
impl Bytecode for I32NotEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_NE])
	}
}

pub struct I32LessThanSigned;
impl Bytecode for I32LessThanSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_LT_S])
	}
}

pub struct I32LessThanUnsigned;
impl Bytecode for I32LessThanUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_LT_U])
	}
}

pub struct I32GreaterThanSigned;
impl Bytecode for I32GreaterThanSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_GT_S])
	}
}

pub struct I32GreaterThanUnsigned;
impl Bytecode for I32GreaterThanUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_GT_U])
	}
}

pub struct I32LessOrEqualSigned;
impl Bytecode for I32LessOrEqualSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_LE_S])
	}
}

pub struct I32LessOrEqualUnsigned;
impl Bytecode for I32LessOrEqualUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_LE_U])
	}
}

pub struct I32GreaterOrEqualSigned;
impl Bytecode for I32GreaterOrEqualSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_GE_S])
	}
}

pub struct I32GreaterOrEqualUnsigned;
impl Bytecode for I32GreaterOrEqualUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_GE_U])
	}
}

pub struct I64EqualToZero;
impl Bytecode for I64EqualToZero{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_EQZ])
	}
}

pub struct I64Equal;
impl Bytecode for I64Equal{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_EQ])
	}
}

pub struct I64NotEqual;
impl Bytecode for I64NotEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_NE])
	}
}

pub struct I64LessThanSigned;
impl Bytecode for I64LessThanSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_LT_S])
	}
}

pub struct I64LessThanUnsigned;
impl Bytecode for I64LessThanUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_LT_U])
	}
}

pub struct I64GreaterThanSigned;
impl Bytecode for I64GreaterThanSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_GT_S])
	}
}

pub struct I64GreaterThanUnsigned;
impl Bytecode for I64GreaterThanUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_GT_U])
	}
}

pub struct I64LessOrEqualSigned;
impl Bytecode for I64LessOrEqualSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_LE_S])
	}
}

pub struct I64LessOrEqualUnsigned;
impl Bytecode for I64LessOrEqualUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_LE_U])
	}
}

pub struct I64GreaterOrEqualSigned;
impl Bytecode for I64GreaterOrEqualSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_GE_S])
	}
}

pub struct I64GreaterOrEqualUnsigned;
impl Bytecode for I64GreaterOrEqualUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_GE_U])
	}
}

pub struct F32Equal;
impl Bytecode for F32Equal{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_EQ])
	}
}

pub struct F32NotEqual;
impl Bytecode for F32NotEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_NE])
	}
}

pub struct F32LessThan;
impl Bytecode for F32LessThan{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_LT])
	}
}

pub struct F32GreaterThan;
impl Bytecode for F32GreaterThan{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_GT])
	}
}

pub struct F32LessOrEqual;
impl Bytecode for F32LessOrEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_LE])
	}
}

pub struct F32GreaterOrEqual;
impl Bytecode for F32GreaterOrEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_GE])
	}
}

pub struct F64Equal;
impl Bytecode for F64Equal{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_EQ])
	}
}

pub struct F64NotEqual;
impl Bytecode for F64NotEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_NE])
	}
}

pub struct F64LessThan;
impl Bytecode for F64LessThan{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_LT])
	}
}

pub struct F64GreaterThan;
impl Bytecode for F64GreaterThan{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_GT])
	}
}

pub struct F64LessOrEqual;
impl Bytecode for F64LessOrEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_LE])
	}
}

pub struct F64GreaterOrEqual;
impl Bytecode for F64GreaterOrEqual{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_GE])
	}
}

