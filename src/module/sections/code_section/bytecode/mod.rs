use std::io::Write;
use std::io;
use BlockType;
use leb128;

pub trait Bytecode {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()>;
}

const RESERVED_VALUE: u8 = 0;

//Control Flow
const UNREACHABLE: u8 = 0x00;
const NOOP: u8 = 0x01;
const BLOCK: u8 = 0x02;
const LOOP: u8 = 0x03;
const IF: u8 = 0x04;
const ELSE: u8 = 0x05;
const END: u8 = 0x0b;
const BR: u8 = 0x0c;
const BR_IF: u8 = 0x0d;
const BR_TABLE: u8 = 0x0e;
const RETURN: u8 = 0x0f;

//call
const CALL: u8 = 0x10;
const CALL_INDIRECT: u8 = 0x11;

//stack
const DROP: u8 = 0x1a;
const SELECT: u8 = 0x1b;

//variable
const GET_LOCAL: u8 = 0x20;
const SET_LOCAL: u8 = 0x21;
const TEE_LOCAL: u8 = 0x22;
const GET_GLOBAL: u8 = 0x23;
const SET_GLOBAL: u8 = 0x24;

//memory
const I32_LOAD: u8 = 0x28;
const I64_LOAD: u8 = 0x29;
const F32_LOAD: u8 = 0x2a;
const F64_LOAD: u8 = 0x2b;
const I32_LOAD_8_S: u8 = 0x2c;
const I32_LOAD_8_U: u8 = 0x2d;
const I32_LOAD_16_S: u8 = 0x2e;
const I32_LOAD_16_U: u8 = 0x2f;
const I64_LOAD_8_S: u8 = 0x30;
const I64_LOAD_8_U: u8 = 0x31;
const I64_LOAD_16_S: u8 = 0x32;
const I64_LOAD_16_U: u8 = 0x33;
const I64_LOAD_32_S: u8 = 0x34;
const I64_LOAD_32_U: u8 = 0x35;

const I32_STORE: u8 = 0x36;
const I64_STORE: u8 = 0x37;
const F32_STORE: u8 = 0x38;
const F64_STORE: u8 = 0x39;
const I32_STORE_8: u8 = 0x3a;
const I32_STORE_16: u8 = 0x3b;
const I64_STORE_8: u8 = 0x3c;
const I64_STORE_16: u8 = 0x3d;
const I64_STORE_32: u8 = 0x3e;
const CURRENT_MEMORY: u8 = 0x3f;
const GROW_MEMORY: u8 = 0x40;

//constant
const I32_CONST: u8 = 0x41;
const I64_CONST: u8 = 0x42;
const F32_CONST: u8 = 0x43;
const F64_CONST: u8 = 0x44;

//comparison
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

//numeric
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

//conversion
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

//reinterpret
const I32_REINTERPRET_F32: u8 = 0xbc;
const I32_REINTERPRET_F64: u8 = 0xbd;
const F32_REINTERPRET_I32: u8 = 0xbe;
const F64_REINTERPRET_I64: u8 = 0xbf;


pub enum AnyBytecode {
	Unreachable,
	NoOp,

	//TODO: auto calculate blocktype
	Block(Vec<AnyBytecode>, BlockType),
	//TODO: auto calculate blocktype
	Loop(Vec<AnyBytecode>, BlockType),
	//TODO: auto calculate blocktype
	If(Vec<AnyBytecode>, Vec<AnyBytecode>, BlockType),

	Break(u32),
	BreakIf(u32),
	BreakTable(Vec<u32>, u32),
	Return,

	Call(u32 /* function index */),
	CallIndirect(u32 /* type index */),

	Drop,
	Select,

	GetLocal(u32 /* local index */),
	SetLocal(u32 /* local index */),
	TeeLocal(u32 /* local index */),
	GetGlobal(u32 /* global index */),
	SetGlobal(u32 /* global index */),

	I32Load(u32 /* flags */, u32 /* offset */),
	I64Load(u32 /* flags */, u32 /* offset */),
	F32Load(u32 /* flags */, u32 /* offset */),
	F64Load(u32 /* flags */, u32 /* offset */),
	I32Load8S(u32 /* flags */, u32 /* offset */),
	I32Load8U(u32 /* flags */, u32 /* offset */),
	I32Load16S(u32 /* flags */, u32 /* offset */),
	I32Load16U(u32 /* flags */, u32 /* offset */),
	I64Load8S(u32 /* flags */, u32 /* offset */),
	I64Load8U(u32 /* flags */, u32 /* offset */),
	I64Load16S(u32 /* flags */, u32 /* offset */),
	I64Load16U(u32 /* flags */, u32 /* offset */),
	I64Load32S(u32 /* flags */, u32 /* offset */),
	I64Load32U(u32 /* flags */, u32 /* offset */),

	I32Store(u32 /* flags */, u32 /* offset */),
	I64Store(u32 /* flags */, u32 /* offset */),
	F32Store(u32 /* flags */, u32 /* offset */),
	F64Store(u32 /* flags */, u32 /* offset */),
	I32Store8(u32 /* flags */, u32 /* offset */),
	I32Store16(u32 /* flags */, u32 /* offset */),
	I64Store8(u32 /* flags */, u32 /* offset */),
	I64Store16(u32 /* flags */, u32 /* offset */),
	I64Store32(u32 /* flags */, u32 /* offset */),
	CurrentMemory,
	GrowMemory,

	I32Constant(u32),
	I64Constant(u64),
	F32Constant(f32),
	F64Constant(f64),

	I32EqualToZero,
	I32Equal,
	I32NotEqual,
	I32LessThanSigned,
	I32LessThanUnsigned,
	I32GreaterThanSigned,
	I32GreaterThanUnsigned,
	I32LessOrEqualSigned,
	I32LessOrEqualUnsigned,
	I32GreaterOrEqualSigned,
	I32GreaterOrEqualUnsigned,

	I64EqualToZero,
	I64Equal,
	I64NotEqual,
	I64LessThanSigned,
	I64LessThanUnsigned,
	I64GreaterThanSigned,
	I64GreaterThanUnsigned,
	I64LessOrEqualSigned,
	I64LessOrEqualUnsigned,
	I64GreaterOrEqualSigned,
	I64GreaterOrEqualUnsigned,

	F32Equal,
	F32NotEqual,
	F32LessThan,
	F32GreaterThan,
	F32LessOrEqual,
	F32GreaterOrEqual,

	F64Equal,
	F64NotEqual,
	F64LessThan,
	F64GreaterThan,
	F64LessOrEqual,
	F64GreaterOrEqual,

	I32CountLeadingZeros,
	I32CountTrailingZeros,
	I32PopulationCount,
	I32Add,
	I32Subtract,
	I32Multiply,
	I32DivideSigned,
	I32DivideUnsigned,
	I32RemainderSigned,
	I32RemainderUnsigned,
	I32And,
	I32Or,
	I32Xor,
	I32ShiftLeft,
	I32ShiftRightSigned,
	I32ShiftRightUnsigned,
	I32RotateLeft,
	I32RotateRight,

	I64CountLeadingZeros,
	I64CountTrailingZeros,
	I64PopulationCount,
	I64Add,
	I64Subtract,
	I64Multiply,
	I64DivideSigned,
	I64DivideUnsigned,
	I64RemainderSigned,
	I64RemainderUnsigned,
	I64And,
	I64Or,
	I64Xor,
	I64ShiftLeft,
	I64ShiftRightSigned,
	I64ShiftRightUnsigned,
	I64RotateLeft,
	I64RotateRight,

	F32AbsoluteValue,
	F32Negate,
	F32Ceiling,
	F32Floor,
	F32Truncate,
	F32Nearest,
	F32SquareRoot,
	F32Add,
	F32Subtract,
	F32Multiply,
	F32Divide,
	F32Min,
	F32Max,
	F32CopySign,

	F64AbsoluteValue,
	F64Negate,
	F64Ceiling,
	F64Floor,
	F64Truncate,
	F64Nearest,
	F64SquareRoot,
	F64Add,
	F64Subtract,
	F64Multiply,
	F64Divide,
	F64Min,
	F64Max,
	F64CopySign,

	I32WrapI64,
	I32TruncateSignedF32,
	I32TruncateUnsignedF32,
	I32TruncateSignedF64,
	I32TruncateUnsignedF64,

	I64ExtendSignedI32,
	I64ExtendUnsignedI32,
	I64TruncateSignedF32,
	I64TruncateUnsignedF32,
	I64TruncateSignedF64,
	I64TruncateUnsignedF64,

	F32ConvertSignedI32,
	F32ConvertUnsignedI32,
	F32ConvertSignedI64,
	F32ConvertUnsignedI64,
	F32DemoteF64,

	F64ConvertSignedI32,
	F64ConvertUnsignedI32,
	F64ConvertSignedI64,
	F64ConvertUnsignedI64,
	F64PromoteF32,

	I32ReinterpretF32,
	I32ReinterpretF64,
	F32ReinterpretI32,
	F64ReinterpretI64,

}

impl Bytecode for AnyBytecode {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		match *self {
			AnyBytecode::Unreachable => out.write_all(&[UNREACHABLE]),
			AnyBytecode::NoOp => out.write_all(&[NOOP]),
			AnyBytecode::Block(ref bytecode, ref blocktype) => {
				out.write_all(&[
					BLOCK,
					blocktype.get_bytecode() as u8
				])?;
				bytecode.compile(out)?;
				out.write_all(&[END])
			}
			AnyBytecode::Loop(ref bytecode, ref blocktype) => {
				out.write_all(&[
					LOOP,
					blocktype.get_bytecode() as u8
				])?;
				bytecode.compile(out)?;
				out.write_all(&[END])
			}
			AnyBytecode::If(ref if_code, ref else_code, ref blocktype) => {
				out.write_all(&[
					IF,
					blocktype.get_bytecode() as u8
				])?;
				if_code.compile(out)?;
				if else_code.len() > 0 {
					out.write_all(&[ELSE])?;
					else_code.compile(out)?;
				}
				out.write_all(&[END])
			}
			AnyBytecode::Break(relative_depth) => {
				out.write_all(&[BR])?;
				leb128::write::unsigned(out, relative_depth as u64)?;
				Ok(())
			}
			AnyBytecode::BreakIf(relative_depth) => {
				out.write_all(&[BR_IF])?;
				leb128::write::unsigned(out, relative_depth as u64)?;
				Ok(())
			}
			AnyBytecode::BreakTable(ref targets, default_target) => {
				out.write_all(&[BR_TABLE])?;
				leb128::write::unsigned(out, targets.len() as u64)?;
				for target in targets {
					leb128::write::unsigned(out, *target as u64)?;
				}
				leb128::write::unsigned(out, default_target as u64)?;
				Ok(())
			}
			AnyBytecode::Return => out.write_all(&[RETURN]),

			AnyBytecode::Call(function_index) => {
				out.write_all(&[CALL])?;
				leb128::write::unsigned(out, function_index as u64)?;
				Ok(())
			}
			AnyBytecode::CallIndirect(type_index) => {
				out.write_all(&[CALL_INDIRECT])?;
				leb128::write::unsigned(out, type_index as u64)?;
				leb128::write::unsigned(out, RESERVED_VALUE as u64)?;
				Ok(())
			}

			AnyBytecode::Drop => out.write_all(&[DROP]),
			AnyBytecode::Select => out.write_all(&[SELECT]),

			AnyBytecode::GetLocal(local_index) => {
				out.write_all(&[GET_LOCAL])?;
				leb128::write::unsigned(out, local_index as u64)?;
				Ok(())
			}
			AnyBytecode::SetLocal(local_index) => {
				out.write_all(&[SET_LOCAL])?;
				leb128::write::unsigned(out, local_index as u64)?;
				Ok(())
			}
			AnyBytecode::TeeLocal(local_index) => {
				out.write_all(&[TEE_LOCAL])?;
				leb128::write::unsigned(out, local_index as u64)?;
				Ok(())
			}
			AnyBytecode::GetGlobal(global_index) => {
				out.write_all(&[GET_GLOBAL])?;
				leb128::write::unsigned(out, global_index as u64)?;
				Ok(())
			}
			AnyBytecode::SetGlobal(global_index) => {
				out.write_all(&[SET_GLOBAL])?;
				leb128::write::unsigned(out, global_index as u64)?;
				Ok(())
			}

			AnyBytecode::I32Load(flags, offset) => compile_memory(I32_LOAD, out, flags, offset),
			AnyBytecode::I64Load(flags, offset) => compile_memory(I64_LOAD, out, flags, offset),
			AnyBytecode::F32Load(flags, offset) => compile_memory(F32_LOAD, out, flags, offset),
			AnyBytecode::F64Load(flags, offset) => compile_memory(F64_LOAD, out, flags, offset),
			AnyBytecode::I32Load8S(flags, offset) => compile_memory(I32_LOAD_8_S, out, flags, offset),
			AnyBytecode::I32Load8U(flags, offset) => compile_memory(I32_LOAD_8_U, out, flags, offset),
			AnyBytecode::I32Load16S(flags, offset) => compile_memory(I32_LOAD_16_S, out, flags, offset),
			AnyBytecode::I32Load16U(flags, offset) => compile_memory(I32_LOAD_16_U, out, flags, offset),
			AnyBytecode::I64Load8S(flags, offset) => compile_memory(I64_LOAD_8_S, out, flags, offset),
			AnyBytecode::I64Load8U(flags, offset) => compile_memory(I64_LOAD_8_U, out, flags, offset),
			AnyBytecode::I64Load16S(flags, offset) => compile_memory(I64_LOAD_16_S, out, flags, offset),
			AnyBytecode::I64Load16U(flags, offset) => compile_memory(I64_LOAD_16_U, out, flags, offset),
			AnyBytecode::I64Load32S(flags, offset) => compile_memory(I64_LOAD_32_S, out, flags, offset),
			AnyBytecode::I64Load32U(flags, offset) => compile_memory(I64_LOAD_32_U, out, flags, offset),

			AnyBytecode::I32Store(flags, offset) => compile_memory(I32_STORE, out, flags, offset),
			AnyBytecode::I64Store(flags, offset) => compile_memory(I64_STORE, out, flags, offset),
			AnyBytecode::F32Store(flags, offset) => compile_memory(F32_STORE, out, flags, offset),
			AnyBytecode::F64Store(flags, offset) => compile_memory(F64_STORE, out, flags, offset),
			AnyBytecode::I32Store8(flags, offset) => compile_memory(I32_STORE_8, out, flags, offset),
			AnyBytecode::I32Store16(flags, offset) => compile_memory(I32_STORE_16, out, flags, offset),
			AnyBytecode::I64Store8(flags, offset) => compile_memory(I64_STORE_8, out, flags, offset),
			AnyBytecode::I64Store16(flags, offset) => compile_memory(I64_STORE_16, out, flags, offset),
			AnyBytecode::I64Store32(flags, offset) => compile_memory(I64_STORE_32, out, flags, offset),
			AnyBytecode::CurrentMemory => out.write_all(&[CURRENT_MEMORY]),
			AnyBytecode::GrowMemory => out.write_all(&[GROW_MEMORY]),

			AnyBytecode::I32Constant(value) => {
				out.write_all(&[I32_CONST])?;
				leb128::write::signed(out, value as i64)?;
				Ok(())
			}
			AnyBytecode::I64Constant(value) => {
				out.write_all(&[I64_CONST])?;
				leb128::write::signed(out, value as i64)?;
				Ok(())
			}
			AnyBytecode::F32Constant(value) => {
				out.write_all(&[F32_CONST])?;
				leb128::write::signed(out, value as i64)?;
				Ok(())
			}
			AnyBytecode::F64Constant(value) => {
				out.write_all(&[F64_CONST])?;
				leb128::write::signed(out, value as i64)?;
				Ok(())
			}

			AnyBytecode::I32EqualToZero => out.write_all(&[I32_EQZ]),
			AnyBytecode::I32Equal => out.write_all(&[I32_EQ]),
			AnyBytecode::I32NotEqual => out.write_all(&[I32_NE]),
			AnyBytecode::I32LessThanSigned => out.write_all(&[I32_LE_S]),
			AnyBytecode::I32LessThanUnsigned => out.write_all(&[I32_LE_U]),
			AnyBytecode::I32GreaterThanSigned => out.write_all(&[I32_GT_S]),
			AnyBytecode::I32GreaterThanUnsigned => out.write_all(&[I32_GT_U]),
			AnyBytecode::I32LessOrEqualSigned => out.write_all(&[I32_LE_S]),
			AnyBytecode::I32LessOrEqualUnsigned => out.write_all(&[I32_LE_U]),
			AnyBytecode::I32GreaterOrEqualSigned => out.write_all(&[I32_GE_S]),
			AnyBytecode::I32GreaterOrEqualUnsigned => out.write_all(&[I32_GE_U]),

			AnyBytecode::I64EqualToZero => out.write_all(&[I64_EQZ]),
			AnyBytecode::I64Equal => out.write_all(&[I64_EQ]),
			AnyBytecode::I64NotEqual => out.write_all(&[I64_NE]),
			AnyBytecode::I64LessThanSigned => out.write_all(&[I64_LE_S]),
			AnyBytecode::I64LessThanUnsigned => out.write_all(&[I64_LE_U]),
			AnyBytecode::I64GreaterThanSigned => out.write_all(&[I64_GT_S]),
			AnyBytecode::I64GreaterThanUnsigned => out.write_all(&[I64_GT_U]),
			AnyBytecode::I64LessOrEqualSigned => out.write_all(&[I64_LE_S]),
			AnyBytecode::I64LessOrEqualUnsigned => out.write_all(&[I64_LE_U]),
			AnyBytecode::I64GreaterOrEqualSigned => out.write_all(&[I64_GE_S]),
			AnyBytecode::I64GreaterOrEqualUnsigned => out.write_all(&[I64_GE_U]),

			AnyBytecode::F32Equal => out.write_all(&[F32_EQ]),
			AnyBytecode::F32NotEqual => out.write_all(&[F32_NE]),
			AnyBytecode::F32LessThan => out.write_all(&[F32_LT]),
			AnyBytecode::F32GreaterThan => out.write_all(&[F32_GT]),
			AnyBytecode::F32LessOrEqual => out.write_all(&[F32_LE]),
			AnyBytecode::F32GreaterOrEqual => out.write_all(&[F32_GE]),

			AnyBytecode::F64Equal => out.write_all(&[F64_EQ]),
			AnyBytecode::F64NotEqual => out.write_all(&[F64_NE]),
			AnyBytecode::F64LessThan => out.write_all(&[F64_LT]),
			AnyBytecode::F64GreaterThan => out.write_all(&[F64_GT]),
			AnyBytecode::F64LessOrEqual => out.write_all(&[F64_LE]),
			AnyBytecode::F64GreaterOrEqual => out.write_all(&[F64_GE]),

			AnyBytecode::I32CountLeadingZeros => out.write_all(&[I32_CLZ]),
			AnyBytecode::I32CountTrailingZeros => out.write_all(&[I32_CTZ]),
			AnyBytecode::I32PopulationCount => out.write_all(&[I32_POPCNT]),
			AnyBytecode::I32Add => out.write_all(&[I32_ADD]),
			AnyBytecode::I32Subtract => out.write_all(&[I32_SUB]),
			AnyBytecode::I32Multiply => out.write_all(&[I32_MUL]),
			AnyBytecode::I32DivideSigned => out.write_all(&[I32_DIV_S]),
			AnyBytecode::I32DivideUnsigned => out.write_all(&[I32_DIV_U]),
			AnyBytecode::I32RemainderSigned => out.write_all(&[I32_REM_S]),
			AnyBytecode::I32RemainderUnsigned => out.write_all(&[I32_REM_U]),
			AnyBytecode::I32And => out.write_all(&[I32_AND]),
			AnyBytecode::I32Or => out.write_all(&[I32_OR]),
			AnyBytecode::I32Xor => out.write_all(&[I32_XOR]),
			AnyBytecode::I32ShiftLeft => out.write_all(&[I32_SHL]),
			AnyBytecode::I32ShiftRightSigned => out.write_all(&[I32_SHR_S]),
			AnyBytecode::I32ShiftRightUnsigned => out.write_all(&[I32_SHR_U]),
			AnyBytecode::I32RotateLeft => out.write_all(&[I32_ROTL]),
			AnyBytecode::I32RotateRight => out.write_all(&[I32_ROTR]),

			AnyBytecode::I64CountLeadingZeros => out.write_all(&[I64_CLZ]),
			AnyBytecode::I64CountTrailingZeros => out.write_all(&[I64_CTZ]),
			AnyBytecode::I64PopulationCount => out.write_all(&[I64_POPCNT]),
			AnyBytecode::I64Add => out.write_all(&[I64_ADD]),
			AnyBytecode::I64Subtract => out.write_all(&[I64_SUB]),
			AnyBytecode::I64Multiply => out.write_all(&[I64_MUL]),
			AnyBytecode::I64DivideSigned => out.write_all(&[I64_DIV_S]),
			AnyBytecode::I64DivideUnsigned => out.write_all(&[I64_DIV_U]),
			AnyBytecode::I64RemainderSigned => out.write_all(&[I64_REM_S]),
			AnyBytecode::I64RemainderUnsigned => out.write_all(&[I64_REM_U]),
			AnyBytecode::I64And => out.write_all(&[I64_AND]),
			AnyBytecode::I64Or => out.write_all(&[I64_OR]),
			AnyBytecode::I64Xor => out.write_all(&[I64_XOR]),
			AnyBytecode::I64ShiftLeft => out.write_all(&[I64_SHL]),
			AnyBytecode::I64ShiftRightSigned => out.write_all(&[I64_SHR_S]),
			AnyBytecode::I64ShiftRightUnsigned => out.write_all(&[I64_SHR_U]),
			AnyBytecode::I64RotateLeft => out.write_all(&[I64_ROTL]),
			AnyBytecode::I64RotateRight => out.write_all(&[I64_ROTR]),

			AnyBytecode::F32AbsoluteValue => out.write_all(&[F32_ABS]),
			AnyBytecode::F32Negate => out.write_all(&[F32_NEG]),
			AnyBytecode::F32Ceiling => out.write_all(&[F32_CEIL]),
			AnyBytecode::F32Floor => out.write_all(&[F32_FLOOR]),
			AnyBytecode::F32Truncate => out.write_all(&[F32_TRUNC]),
			AnyBytecode::F32Nearest => out.write_all(&[F32_NEAREST]),
			AnyBytecode::F32SquareRoot => out.write_all(&[F32_SQRT]),
			AnyBytecode::F32Add => out.write_all(&[F32_ADD]),
			AnyBytecode::F32Subtract => out.write_all(&[F32_SUB]),
			AnyBytecode::F32Multiply => out.write_all(&[F32_MUL]),
			AnyBytecode::F32Divide => out.write_all(&[F32_DIV]),
			AnyBytecode::F32Min => out.write_all(&[F32_MIN]),
			AnyBytecode::F32Max => out.write_all(&[F32_MAX]),
			AnyBytecode::F32CopySign => out.write_all(&[F32_COPYSIGN]),

			AnyBytecode::F64AbsoluteValue => out.write_all(&[F64_ABS]),
			AnyBytecode::F64Negate => out.write_all(&[F64_NEG]),
			AnyBytecode::F64Ceiling => out.write_all(&[F64_CEIL]),
			AnyBytecode::F64Floor => out.write_all(&[F64_FLOOR]),
			AnyBytecode::F64Truncate => out.write_all(&[F64_TRUNC]),
			AnyBytecode::F64Nearest => out.write_all(&[F64_NEAREST]),
			AnyBytecode::F64SquareRoot => out.write_all(&[F64_SQRT]),
			AnyBytecode::F64Add => out.write_all(&[F64_ADD]),
			AnyBytecode::F64Subtract => out.write_all(&[F64_SUB]),
			AnyBytecode::F64Multiply => out.write_all(&[F64_MUL]),
			AnyBytecode::F64Divide => out.write_all(&[F64_DIV]),
			AnyBytecode::F64Min => out.write_all(&[F64_MIN]),
			AnyBytecode::F64Max => out.write_all(&[F64_MAX]),
			AnyBytecode::F64CopySign => out.write_all(&[F64_COPYSIGN]),

			AnyBytecode::I32WrapI64 => out.write_all(&[I32_WRAP_I64]),
			AnyBytecode::I32TruncateSignedF32 => out.write_all(&[I32_TRUNC_S_F32]),
			AnyBytecode::I32TruncateUnsignedF32 => out.write_all(&[I32_TRUNC_U_F32]),
			AnyBytecode::I32TruncateSignedF64 => out.write_all(&[I32_TRUNC_S_F64]),
			AnyBytecode::I32TruncateUnsignedF64 => out.write_all(&[I32_TRUNC_U_F64]),

			AnyBytecode::I64ExtendSignedI32 => out.write_all(&[I64_EXTEND_S_I32]),
			AnyBytecode::I64ExtendUnsignedI32 => out.write_all(&[I64_EXTEND_U_I32]),
			AnyBytecode::I64TruncateSignedF32 => out.write_all(&[I64_TRUNC_S_F32]),
			AnyBytecode::I64TruncateUnsignedF32 => out.write_all(&[I64_TRUNC_U_F32]),
			AnyBytecode::I64TruncateSignedF64 => out.write_all(&[I64_TRUNC_S_F64]),
			AnyBytecode::I64TruncateUnsignedF64 => out.write_all(&[I64_TRUNC_U_F64]),

			AnyBytecode::F32ConvertSignedI32 => out.write_all(&[F32_CONVERT_S_I32]),
			AnyBytecode::F32ConvertUnsignedI32 => out.write_all(&[F32_CONVERT_U_I32]),
			AnyBytecode::F32ConvertSignedI64 => out.write_all(&[F32_CONVERT_S_I64]),
			AnyBytecode::F32ConvertUnsignedI64 => out.write_all(&[F32_CONVERT_U_I64]),
			AnyBytecode::F32DemoteF64 => out.write_all(&[F32_DEMOTE_F64]),

			AnyBytecode::F64ConvertSignedI32 => out.write_all(&[F64_CONVERT_S_I32]),
			AnyBytecode::F64ConvertUnsignedI32 => out.write_all(&[F64_CONVERT_U_I32]),
			AnyBytecode::F64ConvertSignedI64 => out.write_all(&[F64_CONVERT_S_I64]),
			AnyBytecode::F64ConvertUnsignedI64 => out.write_all(&[F64_CONVERT_U_I64]),
			AnyBytecode::F64PromoteF32 => out.write_all(&[F64_PROMOTE_F32]),

			AnyBytecode::I32ReinterpretF32 => out.write_all(&[I32_REINTERPRET_F32]),
			AnyBytecode::I32ReinterpretF64 => out.write_all(&[I32_REINTERPRET_F64]),
			AnyBytecode::F32ReinterpretI32 => out.write_all(&[F32_REINTERPRET_I32]),
			AnyBytecode::F64ReinterpretI64 => out.write_all(&[F64_REINTERPRET_I64]),
		}
	}
}

impl Bytecode for Vec<AnyBytecode> {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		for bytecode in self {
			bytecode.compile(out)?;
		}
		Ok(())
	}
}

fn compile_memory<W: Write>(bytecode: u8, out: &mut W, flags: u32, offset: u32) -> io::Result<()> {
	out.write_all(&[bytecode])?;
	leb128::write::unsigned(out, flags as u64)?;
	leb128::write::unsigned(out, offset as u64)?;
	Ok(())
}





