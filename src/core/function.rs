use super::statement::Statement;
use super::ty::Type;

#[derive(Clone)]
pub struct Function {
	statements: Vec<Statement>
}

impl Function {
	pub fn new(statements: Vec<Statement>) -> Function{
		Function{
			statements: statements.to_vec()
		}
	}

	pub fn get_statements(&self) -> &Vec<Statement>{
		&self.statements
	}

	pub fn get_type(&self) -> Type{
		self.statements.last().unwrap().get_type()
	}
}