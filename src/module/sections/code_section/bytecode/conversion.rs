use super::Bytecode;
use std::io::Write;
use std::io;

const I32_WRAP_I64: u8 = 0xa7;
const I32_TRUNC_S_F32: u8 = 0xa8;
const I32_TRUNC_U_F32: u8 = 0xa9;
const I32_TRUNC_S_F64: u8 = 0xaa;
const I32_TRUNC_U_F64: u8 = 0xab;

const I64_EXTEND_S_I32: u8 = 0xac;
const I64_EXTEND_U_I32: u8 = 0xad;
const I64_TRUNC_S_F32: u8 = 0xae;
const I64_TRUNC_U_F32: u8 = 0xaf;
const I64_TRUNC_S_F64: u8 = 0xb0;
const I64_TRUNC_U_F64: u8 = 0xb1;

const F32_CONVERT_S_I32: u8 = 0xb2;
const F32_CONVERT_U_I32: u8 = 0xb3;
const F32_CONVERT_S_I64: u8 = 0xb4;
const F32_CONVERT_U_I64: u8 = 0xb5;
const F32_DEMOTE_F64: u8 = 0xb6;

const F64_CONVERT_S_I32: u8 = 0xb7;
const F64_CONVERT_U_I32: u8 = 0xb8;
const F64_CONVERT_S_I64: u8 = 0xb9;
const F64_CONVERT_U_I64: u8 = 0xba;
const F64_PROMOTE_F32: u8 = 0xbb;

pub struct I32WrapI64;

impl Bytecode for I32WrapI64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_WRAP_I64])
	}
}

pub struct I32TruncateSignedF32;

impl Bytecode for I32TruncateSignedF32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_TRUNC_S_F32])
	}
}

pub struct I32TruncateUnsignedF32;

impl Bytecode for I32TruncateUnsignedF32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_TRUNC_U_F32])
	}
}

pub struct I32TruncateSignedF64;

impl Bytecode for I32TruncateSignedF64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_TRUNC_S_F64])
	}
}

pub struct I32TruncateUnsignedF64;

impl Bytecode for I32TruncateUnsignedF64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_TRUNC_U_F64])
	}
}

pub struct I64ExtendSignedI32;

impl Bytecode for I64ExtendSignedI32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_EXTEND_S_I32])
	}
}

pub struct I64ExtendUnsignedI32;

impl Bytecode for I64ExtendUnsignedI32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_EXTEND_U_I32])
	}
}

pub struct I64TruncateSignedF32;

impl Bytecode for I64TruncateSignedF32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_TRUNC_S_F32])
	}
}

pub struct I64TruncateUnsignedF32;

impl Bytecode for I64TruncateUnsignedF32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_TRUNC_U_F32])
	}
}

pub struct I64TruncateSignedF64;

impl Bytecode for I64TruncateSignedF64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_TRUNC_S_F64])
	}
}

pub struct I64TruncateUnsignedF64;

impl Bytecode for I64TruncateUnsignedF64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_TRUNC_U_F64])
	}
}

pub struct F32ConvertSignedI32;

impl Bytecode for F32ConvertSignedI32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_CONVERT_S_I32])
	}
}

pub struct F32ConvertUnsignedI32;

impl Bytecode for F32ConvertUnsignedI32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_CONVERT_U_I32])
	}
}

pub struct F32ConvertSignedI64;

impl Bytecode for F32ConvertSignedI64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_CONVERT_S_I64])
	}
}

pub struct F32ConvertUnsignedI64;

impl Bytecode for F32ConvertUnsignedI64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_CONVERT_U_I64])
	}
}

pub struct F32DemoteF64;

impl Bytecode for F32DemoteF64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_DEMOTE_F64])
	}
}

pub struct F64ConvertSignedI32;

impl Bytecode for F64ConvertSignedI32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_CONVERT_S_I32])
	}
}

pub struct F64ConvertUnsignedI32;

impl Bytecode for F64ConvertUnsignedI32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_CONVERT_U_I32])
	}
}

pub struct F64ConvertSignedI64;

impl Bytecode for F64ConvertSignedI64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_CONVERT_S_I64])
	}
}

pub struct F64ConvertUnsignedI64;

impl Bytecode for F64ConvertUnsignedI64 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_CONVERT_U_I64])
	}
}

pub struct F64PromoteF32;

impl Bytecode for F64PromoteF32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_PROMOTE_F32])
	}
}





