use super::{AnyBytecode, Bytecode};
use ty::BlockType;
use leb128;
use std::io::Write;
use std::io;

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


pub struct Unreachable;

impl Bytecode for Unreachable {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[UNREACHABLE])
	}
}

pub struct NoOp;

impl Bytecode for NoOp {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[NOOP])
	}
}

pub struct Block {
	inner_bytecode: Vec<AnyBytecode>,
	signature: BlockType
}

impl Bytecode for Block {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[
			BLOCK,
			self.signature.get_bytecode() as u8
		])?;
		self.inner_bytecode.compile(out);
		out.write_all(&[END])
	}
}

pub struct Loop {
	inner_bytecode: Vec<AnyBytecode>,
	signature: BlockType
}

impl Bytecode for Loop {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[
			LOOP,
			self.signature.get_bytecode() as u8
		]);
		self.inner_bytecode.compile(out);
		out.write_all(&[END])
	}
}

pub struct If {
	if_bytecode: Vec<AnyBytecode>,
	else_bytecode: Vec<AnyBytecode>,
	signature: BlockType
}

impl Bytecode for If {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[
			IF,
			self.signature.get_bytecode() as u8
		])?;
		self.if_bytecode.compile(out)?;
		if self.else_bytecode.len() > 0{
			out.write_all(&[ELSE])?;
			self.else_bytecode.compile(out)?;
		}
		out.write_all(&[END])
	}
}

pub struct Break {
	relative_depth: u32//varuint32
}

impl Bytecode for Break {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[BR]);
		leb128::write::unsigned(out, self.relative_depth as u64)?;
		Ok(())
	}
}

pub struct BreakIf {
	relative_depth: u32//varuint32
}

impl Bytecode for BreakIf {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[BR_IF]);
		leb128::write::unsigned(out, self.relative_depth as u64)?;
		Ok(())
	}
}

pub struct BreakTable {
	targets: Vec<u32>,
	//varuint32...
	default_target: u32 //varuint32
}

impl Bytecode for BreakTable {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[BR_TABLE])?;
		leb128::write::unsigned(out, self.targets.len() as u64)?;
		for target in &self.targets {
			leb128::write::unsigned(out, *target as u64)?;
		}
		leb128::write::unsigned(out, self.default_target as u64)?;
		Ok(())
	}
}

pub struct Return;

impl Bytecode for Return {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[RETURN])
	}
}
