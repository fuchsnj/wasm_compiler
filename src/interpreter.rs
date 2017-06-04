use core::*;

pub struct Interpreter;

impl Interpreter {
	pub fn run(&self, program: &Program) {
		let main_func = program.get_main();
		self.run_func(&main_func);
	}

	pub fn run_func(&self, func: &Function) -> TypedValue{
		let mut value = TypedValue::Void;
		for statement in func.get_statements() {
			value = self.run_statement(&statement);
		}
		value
	}

	pub fn run_statement(&self, statement: &Statement) -> TypedValue {
		match *statement {
			Statement::CallFunction(ref func) => {
				self.run_func(&func)
			},
			Statement::Log(ref msg) => {
				println!("{}", msg);
				TypedValue::Void
			},
			Statement::NoOp => {
				TypedValue::Void
			}
			Statement::Value(ref val) => {
				val.clone()
			}
			Statement::Add(ref a, ref b) => {
				let a_value = self.run_statement(a);
				let b_value = self.run_statement(b);
				match (a_value, b_value){
					(TypedValue::I64(a), TypedValue::I64(b)) => TypedValue::I64(a+b),
					(a, b) => {
						panic!("Cannot add values of type {:?} and {:?}",
						a.get_type(), b.get_type())
					}
				}
			}
			Statement::IfElse(ref condition, ref a, ref b) => {
				let condition_value = self.run_statement(condition);
				if let TypedValue::Boolean(bool_value) = condition_value{
					if bool_value{
						self.run_statement(a)
					}else{
						self.run_statement(b)
					}
				}else{
					panic!("if/else condition must be a boolean");
				}

			}
		}
	}
}