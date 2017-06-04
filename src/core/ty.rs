#[derive(Debug, PartialEq)]
pub enum Type {
	Void,
	I64,
	Boolean
}

#[derive(Clone)]
pub enum TypedValue{
	Void,
	I64(i64),
	Boolean(bool)
}

impl TypedValue{
	pub fn get_type(&self) -> Type{
		match *self{
			TypedValue::Void => Type::Void,
			TypedValue::I64(_) => Type::I64,
			TypedValue::Boolean(_) => Type::Boolean
		}
	}
}