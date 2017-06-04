use super::ty::TypedValue;

pub struct Variable {
	value: TypedValue
}

impl Variable {
	pub fn new(value: TypedValue) -> Variable {
		Variable { value }
	}

	pub fn set_value(&mut self, value: TypedValue){
		assert_eq!(self.value.get_type(), value.get_type());
		self.value = value;
	}

	pub fn get_value(&self) -> TypedValue{
		self.value.clone()
	}
}
