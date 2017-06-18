use super::Bytecode;
use std::io::Write;
use std::io;

const DROP: u8 = 0x1a;
const SELECT: u8 = 0x1b;

pub struct Drop;

impl Bytecode for Drop {
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[DROP])
	}
}

pub struct Select;

impl Bytecode for Select{
	fn compile<W: Write>(&self, out: &mut W) -> io::Result<()> {
		out.write_all(&[SELECT])
	}
}

