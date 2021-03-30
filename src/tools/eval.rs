// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;
use gl_runtime::preludes::*;

pub fn run<T: Into<String>>(source: T, module: T, program: &mut ProgramState) -> crate::ResultCli {
	let module: String = module.into();
	let source: Source = Source::from_string(source.into());
	let lexer: Lexer = Lexer::new(source, &module);
	let parser: Parser = Parser::new(lexer)?;
	let runtime: Runtime = Runtime::from_env(Rc::clone(&program.modules[&module]), &module);
	let object: Object = runtime.run_with_parser(parser)?;

	match object {
		Object::Null => {},
		object => println!("{}", object),
	}

	Ok(())
}
