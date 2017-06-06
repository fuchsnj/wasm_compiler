#[derive(Clone, Copy)]
pub enum Type{
	I32 = -0x01,
	I64 = -0x02,
	F32 = -0x03,
	F64 = -0x04,
	AnyFunc = -0x10,
	Func = -0x20,
	EmptyBlockType = -0x40
}

pub enum ValueType{
	I32 = Type::I32 as isize,
	I64 = Type::I64 as isize,
	F32 = Type::F32 as isize,
	F64 = Type::F64 as isize
}