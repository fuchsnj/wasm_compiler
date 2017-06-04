use super::Type;
use std::io::Write;
use leb128;

pub struct FunctionSignature{
	param_types: Vec<Type>,
	return_type: Option<Type>
}
impl FunctionSignature{
	pub fn new(param_types: Vec<Type>, return_type: Option<Type>) -> FunctionSignature{
		FunctionSignature{
			param_types,
			return_type
		}
	}

	pub fn compile<W: Write>(&self, out: &mut W){
		leb128::write::signed(out, Type::Func as u64 as i64);//Func type constructor id
		leb128::write::unsigned(out, self.param_types.len() as u64);//num of parameter types
		for param_type in &self.param_types{
			leb128::write::signed(out, *param_type as u64 as i64);//param type id
		}
		if let Some(return_type) = self.return_type{
			leb128::write::unsigned(out, 1);
			leb128::write::signed(out, return_type as u64 as i64);//return type id
		}else{
			leb128::write::unsigned(out, 0);
		}
	}
}