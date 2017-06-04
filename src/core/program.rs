use super::function::Function;

pub struct Program {
	//TODO: make private
	pub main: Function
}

impl Program {
	pub fn get_main(&self) -> &Function{
		&self.main
	}
}