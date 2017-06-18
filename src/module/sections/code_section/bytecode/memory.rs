use super::Bytecode;
use std::io::Write;
use std::io;
use leb128;

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

const RESERVED: u8 = 0;

pub struct MemoryImmediate {
	flags: u32,
	offset: u32
}

fn compile<W: Write>(bytecode: u8, out: &mut W, immediate: &MemoryImmediate) -> io::Result<()> {
	out.write_all(&[bytecode])?;
	leb128::write::unsigned(out, immediate.flags as u64)?;
	leb128::write::unsigned(out, immediate.offset as u64)?;
	Ok(())
}

pub struct I32Load {
	immediate: MemoryImmediate
}

impl Bytecode for I32Load {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_LOAD, out, &self.immediate)
	}
}

pub struct I64Load {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD, out, &self.immediate)
	}
}

pub struct F32Load {
	immediate: MemoryImmediate
}

impl Bytecode for F32Load {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(F32_LOAD, out, &self.immediate)
	}
}

pub struct F64Load {
	immediate: MemoryImmediate
}

impl Bytecode for F64Load {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(F64_LOAD, out, &self.immediate)
	}
}

pub struct I32Load8S {
	immediate: MemoryImmediate
}

impl Bytecode for I32Load8S {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_LOAD_8_S, out, &self.immediate)
	}
}

pub struct I32Load8U {
	immediate: MemoryImmediate
}

impl Bytecode for I32Load8U {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_LOAD_8_U, out, &self.immediate)
	}
}

pub struct I32Load16S {
	immediate: MemoryImmediate
}

impl Bytecode for I32Load16S {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_LOAD_16_S, out, &self.immediate)
	}
}

pub struct I32Load16U {
	immediate: MemoryImmediate
}

impl Bytecode for I32Load16U {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_LOAD_16_U, out, &self.immediate)
	}
}

pub struct I64Load8S {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load8S {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD_8_S, out, &self.immediate)
	}
}

pub struct I64Load8U {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load8U {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD_8_U, out, &self.immediate)
	}
}

pub struct I64Load16S {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load16S {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD_16_S, out, &self.immediate)
	}
}

pub struct I64Load16U {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load16U {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD_16_U, out, &self.immediate)
	}
}

pub struct I64Load32S {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load32S {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD_32_S, out, &self.immediate)
	}
}

pub struct I64Load32U {
	immediate: MemoryImmediate
}

impl Bytecode for I64Load32U {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_LOAD_32_U, out, &self.immediate)
	}
}

pub struct I32Store {
	immediate: MemoryImmediate
}

impl Bytecode for I32Store {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_STORE, out, &self.immediate)
	}
}

pub struct I64Store {
	immediate: MemoryImmediate
}

impl Bytecode for I64Store {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_STORE, out, &self.immediate)
	}
}

pub struct F32Store {
	immediate: MemoryImmediate
}

impl Bytecode for F32Store {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(F32_STORE, out, &self.immediate)
	}
}

pub struct F64Store {
	immediate: MemoryImmediate
}

impl Bytecode for F64Store {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(F64_STORE, out, &self.immediate)
	}
}

pub struct I32Store8 {
	immediate: MemoryImmediate
}

impl Bytecode for I32Store8 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_STORE_8, out, &self.immediate)
	}
}

pub struct I32Store16 {
	immediate: MemoryImmediate
}

impl Bytecode for I32Store16 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I32_STORE_16, out, &self.immediate)
	}
}

pub struct I64Store8 {
	immediate: MemoryImmediate
}

impl Bytecode for I64Store8 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_STORE_8, out, &self.immediate)
	}
}

pub struct I64Store16 {
	immediate: MemoryImmediate
}

impl Bytecode for I64Store16 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_STORE_16, out, &self.immediate)
	}
}

pub struct I64Store32 {
	immediate: MemoryImmediate
}

impl Bytecode for I64Store32 {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		compile(I64_STORE_32, out, &self.immediate)
	}
}

pub struct CurrentMemory {
	immediate: MemoryImmediate
}

impl Bytecode for CurrentMemory {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[CURRENT_MEMORY, RESERVED])
	}
}

pub struct GrowMemory {
	immediate: MemoryImmediate
}

impl Bytecode for GrowMemory {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[GROW_MEMORY, RESERVED])
	}
}





























