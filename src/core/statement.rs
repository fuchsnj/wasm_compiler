use core::ty::{Type, TypedValue};
use core::function::Function;

#[derive(Clone)]
pub enum Statement {
	NoOp,
	IfElse(Box<Statement>, Box<Statement>, Box<Statement>),
	Value(TypedValue),
	Add(Box<Statement>, Box<Statement>),
	Log(String),
	CallFunction(Box<Function>),
	ReadVariable()
}

impl Statement {
	pub fn get_type(&self) -> Type{
		match *self{
			Statement::NoOp => Type::Void,
			Statement::IfElse(ref condition, ref true_val, ref false_val) => true_val.get_type(),
			Statement::Value(ref value) => value.get_type(),
			Statement::Add(ref value_a, ref value_b) => value_a.get_type(),
			Statement::Log(_) => Type::Void,
			Statement::CallFunction(ref func) => func.get_type()
		}
	}
}