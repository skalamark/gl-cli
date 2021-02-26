// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;
use gl_core::state::ProgramState;
use gl_runtime::preludes::*;

pub fn run(source: String, module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let source: Source = Source::from_string(source).unwrap();
	let lexer: Lexer = Lexer::new(source, module);
	let parser: Parser = Parser::new(lexer);
	let runtime: Runtime =
		Runtime::from_env(Rc::clone(&program.env.modules[module]), module.clone());
	let object: Object = match runtime.run_with_parser(parser) {
		Ok(object) => object,
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	match object {
		Object::Null => {}
		object => println!("{}", object),
	}

	Ok(())
}
