pub mod control_flow;
pub mod call;
pub mod parametric;
pub mod variable;
pub mod memory;
pub mod constant;

use std::io::Write;
use std::io;

pub trait Bytecode {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()>;
}

pub enum AnyBytecode {
	Unreachable(control_flow::Unreachable),
	NoOp(control_flow::NoOp),
	Block(control_flow::Block),
	Loop(control_flow::Loop),
	If(control_flow::If),
	Break(control_flow::Break),
	BreakIf(control_flow::BreakIf),
	BreakTable(control_flow::BreakTable),
	Return(control_flow::Return),

	Call(call::Call),
	CallIndirect(call::CallIndirect),

	Drop(parametric::Drop),
	Select(parametric::Select),

	GetLocal(variable::GetLocal),
	SetLocal(variable::SetLocal),
	TeeLocal(variable::TeeLocal),
	GetGlobal(variable::GetGlobal),
	SetGlobal(variable::SetGlobal),

	I32Load(memory::I32Load),
	I64Load(memory::I64Load),
	F32Load(memory::F32Load),
	F64Load(memory::F64Load),
	I32Load8S(memory::I32Load8S),
	I32Load8U(memory::I32Load8U),
	I32Load16S(memory::I32Load16S),
	I32Load16U(memory::I32Load16U),
	I64Load8S(memory::I64Load8S),
	I64Load8U(memory::I64Load8U),
	I64Load16S(memory::I64Load16S),
	I64Load16U(memory::I64Load16U),
	I64Load32S(memory::I64Load32S),
	I64Load32U(memory::I64Load32U),

	I32Store(memory::I32Store),
	I64Store(memory::I64Store),
	F32Store(memory::F32Store),
	F64Store(memory::F64Store),
	I32Store8(memory::I32Store8),
	I32Store16(memory::I32Store16),
	I64Store8(memory::I64Store8),
	I64Store16(memory::I64Store16),
	I64Store32(memory::I64Store32),
	CurrentMemory(memory::CurrentMemory),
	GrowMemory(memory::GrowMemory),

	I32Constant(constant::I32Constant),
	I64Constant(constant::I64Constant),
	F32Constant(constant::F32Constant),
	F64Constant(constant::F64Constant),

}

impl Bytecode for AnyBytecode {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		match *self {
			AnyBytecode::Unreachable(ref x) => x.compile(out),
			AnyBytecode::NoOp(ref x) => x.compile(out),
			AnyBytecode::Block(ref x) => x.compile(out),
			AnyBytecode::Loop(ref x) => x.compile(out),
			AnyBytecode::If(ref x) => x.compile(out),
			AnyBytecode::Break(ref x) => x.compile(out),
			AnyBytecode::BreakIf(ref x) => x.compile(out),
			AnyBytecode::BreakTable(ref x) => x.compile(out),
			AnyBytecode::Return(ref x) => x.compile(out),

			AnyBytecode::Call(ref x) => x.compile(out),
			AnyBytecode::CallIndirect(ref x) => x.compile(out),

			AnyBytecode::Drop(ref x) => x.compile(out),
			AnyBytecode::Select(ref x) => x.compile(out),

			AnyBytecode::GetLocal(ref x) => x.compile(out),
			AnyBytecode::SetLocal(ref x) => x.compile(out),
			AnyBytecode::TeeLocal(ref x) => x.compile(out),
			AnyBytecode::GetGlobal(ref x) => x.compile(out),
			AnyBytecode::SetGlobal(ref x) => x.compile(out),

			AnyBytecode::I32Load(ref x) => x.compile(out),
			AnyBytecode::I64Load(ref x) => x.compile(out),
			AnyBytecode::F32Load(ref x) => x.compile(out),
			AnyBytecode::F64Load(ref x) => x.compile(out),
			AnyBytecode::I32Load8S(ref x) => x.compile(out),
			AnyBytecode::I32Load8U(ref x) => x.compile(out),
			AnyBytecode::I32Load16S(ref x) => x.compile(out),
			AnyBytecode::I32Load16U(ref x) => x.compile(out),
			AnyBytecode::I64Load8S(ref x) => x.compile(out),
			AnyBytecode::I64Load8U(ref x) => x.compile(out),
			AnyBytecode::I64Load16S(ref x) => x.compile(out),
			AnyBytecode::I64Load16U(ref x) => x.compile(out),
			AnyBytecode::I64Load32S(ref x) => x.compile(out),
			AnyBytecode::I64Load32U(ref x) => x.compile(out),

			AnyBytecode::I32Store(ref x) => x.compile(out),
			AnyBytecode::I64Store(ref x) => x.compile(out),
			AnyBytecode::F32Store(ref x) => x.compile(out),
			AnyBytecode::F64Store(ref x) => x.compile(out),
			AnyBytecode::I32Store8(ref x) => x.compile(out),
			AnyBytecode::I32Store16(ref x) => x.compile(out),
			AnyBytecode::I64Store8(ref x) => x.compile(out),
			AnyBytecode::I64Store16(ref x) => x.compile(out),
			AnyBytecode::I64Store32(ref x) => x.compile(out),
			AnyBytecode::CurrentMemory(ref x) => x.compile(out),
			AnyBytecode::GrowMemory(ref x) => x.compile(out),

			AnyBytecode::I32Constant(ref x) => x.compile(out),
			AnyBytecode::I64Constant(ref x) => x.compile(out),
			AnyBytecode::F32Constant(ref x) => x.compile(out),
			AnyBytecode::F64Constant(ref x) => x.compile(out),
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





