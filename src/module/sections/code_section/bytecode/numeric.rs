use super::Bytecode;
use std::io::Write;
use std::io;

const I32_CLZ: u8 = 0x67;
const I32_CTZ: u8 = 0x68;
const I32_POPCNT: u8 = 0x69;
const I32_ADD: u8 = 0x6a;
const I32_SUB: u8 = 0x6b;
const I32_MUL: u8 = 0x6c;
const I32_DIV_S: u8 = 0x6d;
const I32_DIV_U: u8 = 0x6e;
const I32_REM_S: u8 = 0x6f;
const I32_REM_U: u8 = 0x70;
const I32_AND: u8 = 0x71;
const I32_OR: u8 = 0x72;
const I32_XOR: u8 = 0x73;
const I32_SHL: u8 = 0x74;
const I32_SHR_S: u8 = 0x75;
const I32_SHR_U: u8 = 0x76;
const I32_ROTL: u8 = 0x77;
const I32_ROTR: u8 = 0x78;

const I64_CLZ: u8 = 0x79;
const I64_CTZ: u8 = 0x7a;
const I64_POPCNT: u8 = 0x7b;
const I64_ADD: u8 = 0x7c;
const I64_SUB: u8 = 0x7d;
const I64_MUL: u8 = 0x7e;
const I64_DIV_S: u8 = 0x7f;
const I64_DIV_U: u8 = 0x80;
const I64_REM_S: u8 = 0x81;
const I64_REM_U: u8 = 0x82;
const I64_AND: u8 = 0x83;
const I64_OR: u8 = 0x84;
const I64_XOR: u8 = 0x85;
const I64_SHL: u8 = 0x86;
const I64_SHR_S: u8 = 0x87;
const I64_SHR_U: u8 = 0x88;
const I64_ROTL: u8 = 0x89;
const I64_ROTR: u8 = 0x8a;

const F32_ABS: u8 = 0x8b;
const F32_NEG: u8 = 0x8c;
const F32_CEIL: u8 = 0x8d;
const F32_FLOOR: u8 = 0x8e;
const F32_TRUNC: u8 = 0x8f;
const F32_NEAREST: u8 = 0x90;
const F32_SQRT: u8 = 0x91;
const F32_ADD: u8 = 0x92;
const F32_SUB: u8 = 0x93;
const F32_MUL: u8 = 0x94;
const F32_DIV: u8 = 0x95;
const F32_MIN: u8 = 0x96;
const F32_MAX: u8 = 0x97;
const F32_COPYSIGN: u8 = 0x98;

const F64_ABS: u8 = 0x99;
const F64_NEG: u8 = 0x9a;
const F64_CEIL: u8 = 0x9b;
const F64_FLOOR: u8 = 0x9c;
const F64_TRUNC: u8 = 0x9d;
const F64_NEAREST: u8 = 0x9e;
const F64_SQRT: u8 = 0x9f;
const F64_ADD: u8 = 0xa0;
const F64_SUB: u8 = 0xa1;
const F64_MUL: u8 = 0xa2;
const F64_DIV: u8 = 0xa3;
const F64_MIN: u8 = 0xa4;
const F64_MAX: u8 = 0xa5;
const F64_COPYSIGN: u8 = 0xa6;

pub struct I32CountLeadingZeros;
impl Bytecode for I32CountLeadingZeros{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_CLZ])
	}
}

pub struct I32CountTrailingZeros;
impl Bytecode for I32CountTrailingZeros{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_CTZ])
	}
}

pub struct I32PopulationCount;
impl Bytecode for I32PopulationCount{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_POPCNT])
	}
}

pub struct I32Add;
impl Bytecode for I32Add{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_ADD])
	}
}

pub struct I32Subtract;
impl Bytecode for I32Subtract{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_SUB])
	}
}

pub struct I32Multiply;
impl Bytecode for I32Multiply{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_MUL])
	}
}

pub struct I32DivideSigned;
impl Bytecode for I32DivideSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_DIV_S])
	}
}

pub struct I32DivideUnsigned;
impl Bytecode for I32DivideUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_DIV_U])
	}
}

pub struct I32RemainderSigned;
impl Bytecode for I32RemainderSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_REM_S])
	}
}

pub struct I32RemainderUnsigned;
impl Bytecode for I32RemainderUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_REM_U])
	}
}

pub struct I32And;
impl Bytecode for I32And{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_AND])
	}
}

pub struct I32Or;
impl Bytecode for I32Or{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_OR])
	}
}

pub struct I32Xor;
impl Bytecode for I32Xor{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_XOR])
	}
}

pub struct I32ShiftLeft;
impl Bytecode for I32ShiftLeft{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_SHL])
	}
}

pub struct I32ShiftRightSigned;
impl Bytecode for I32ShiftRightSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_SHR_S])
	}
}

pub struct I32ShiftRightUnsigned;
impl Bytecode for I32ShiftRightUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_SHR_U])
	}
}

pub struct I32RotateLeft;
impl Bytecode for I32RotateLeft{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_ROTL])
	}
}

pub struct I32RotateRight;
impl Bytecode for I32RotateRight{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I32_ROTR])
	}
}


pub struct I64CountLeadingZeros;
impl Bytecode for I64CountLeadingZeros{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_CLZ])
	}
}

pub struct I64CountTrailingZeros;
impl Bytecode for I64CountTrailingZeros{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_CTZ])
	}
}

pub struct I64PopulationCount;
impl Bytecode for I64PopulationCount{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_POPCNT])
	}
}

pub struct I64Add;
impl Bytecode for I64Add{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_ADD])
	}
}

pub struct I64Subtract;
impl Bytecode for I64Subtract{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_SUB])
	}
}

pub struct I64Multiply;
impl Bytecode for I64Multiply{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_MUL])
	}
}

pub struct I64DivideSigned;
impl Bytecode for I64DivideSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_DIV_S])
	}
}

pub struct I64DivideUnsigned;
impl Bytecode for I64DivideUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_DIV_U])
	}
}

pub struct I64RemainderSigned;
impl Bytecode for I64RemainderSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_REM_S])
	}
}

pub struct I64RemainderUnsigned;
impl Bytecode for I64RemainderUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_REM_U])
	}
}

pub struct I64And;
impl Bytecode for I64And{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_AND])
	}
}

pub struct I64Or;
impl Bytecode for I64Or{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_OR])
	}
}

pub struct I64Xor;
impl Bytecode for I64Xor{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_XOR])
	}
}

pub struct I64ShiftLeft;
impl Bytecode for I64ShiftLeft{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_SHL])
	}
}

pub struct I64ShiftRightSigned;
impl Bytecode for I64ShiftRightSigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_SHR_S])
	}
}

pub struct I64ShiftRightUnsigned;
impl Bytecode for I64ShiftRightUnsigned{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_SHR_U])
	}
}

pub struct I64RotateLeft;
impl Bytecode for I64RotateLeft{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_ROTL])
	}
}

pub struct I64RotateRight;
impl Bytecode for I64RotateRight{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[I64_ROTR])
	}
}

pub struct F32AbsoluteValue;
impl Bytecode for F32AbsoluteValue{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_ABS])
	}
}

pub struct F32Negate;
impl Bytecode for F32Negate{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_NEG])
	}
}

pub struct F32Ceiling;
impl Bytecode for F32Ceiling{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_CEIL])
	}
}

pub struct F32Floor;
impl Bytecode for F32Floor{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_FLOOR])
	}
}

pub struct F32Truncate;
impl Bytecode for F32Truncate{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_TRUNC])
	}
}

pub struct F32Nearest;
impl Bytecode for F32Nearest{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_NEAREST])
	}
}

pub struct F32SquareRoot;
impl Bytecode for F32SquareRoot{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_SQRT])
	}
}

pub struct F32Add;
impl Bytecode for F32Add{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_ADD])
	}
}
pub struct F32Subtract;
impl Bytecode for F32Subtract{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_SUB])
	}
}
pub struct F32Multiply;
impl Bytecode for F32Multiply{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_MUL])
	}
}

pub struct F32Divide;
impl Bytecode for F32Divide{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_DIV])
	}
}

pub struct F32Min;
impl Bytecode for F32Min{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_MIN])
	}
}

pub struct F32Max;
impl Bytecode for F32Max{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_MAX])
	}
}

pub struct F32CopySign;
impl Bytecode for F32CopySign{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F32_COPYSIGN])
	}
}

pub struct F64AbsoluteValue;
impl Bytecode for F64AbsoluteValue{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_ABS])
	}
}

pub struct F64Negate;
impl Bytecode for F64Negate{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_NEG])
	}
}

pub struct F64Ceiling;
impl Bytecode for F64Ceiling{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_CEIL])
	}
}

pub struct F64Floor;
impl Bytecode for F64Floor{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_FLOOR])
	}
}

pub struct F64Truncate;
impl Bytecode for F64Truncate{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_TRUNC])
	}
}

pub struct F64Nearest;
impl Bytecode for F64Nearest{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_NEAREST])
	}
}

pub struct F64SquareRoot;
impl Bytecode for F64SquareRoot{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_SQRT])
	}
}

pub struct F64Add;
impl Bytecode for F64Add{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_ADD])
	}
}
pub struct F64Subtract;
impl Bytecode for F64Subtract{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_SUB])
	}
}
pub struct F64Multiply;
impl Bytecode for F64Multiply{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_MUL])
	}
}

pub struct F64Divide;
impl Bytecode for F64Divide{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_DIV])
	}
}

pub struct F64Min;
impl Bytecode for F64Min{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_MIN])
	}
}

pub struct F64Max;
impl Bytecode for F64Max{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_MAX])
	}
}

pub struct F64CopySign;
impl Bytecode for F64CopySign{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[F64_COPYSIGN])
	}
}





