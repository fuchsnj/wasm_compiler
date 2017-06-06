use Module;


#[test]
fn test_empty_module() {
	let mut module = Module::new();

	let mut buffer: Vec<u8> = vec!();
	module.compile(&mut buffer);
	assert_eq!(buffer, vec![
		0, b'a', b's', b'm', //magic number
		1, 0, 0, 0//version
	]);
}