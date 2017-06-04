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