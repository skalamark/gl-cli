// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::object::{Null, Object};
use gl_core::parser::Parser;
use gl_core::source::Source;
use gl_core::state::ProgramState;
use gl_runtime::Runtime;
use std::sync::Arc;

pub fn run(source: String, module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let source: Source = Source::new_from_string(source).unwrap();
	let lexer: Lexer = Lexer::new(source, module);
	let parser: Parser = Parser::new(lexer);
	let runtime: Runtime = Runtime::new_from_env(Arc::clone(&program.env.modules[module]), module);
	let object: std::sync::Arc<std::sync::Mutex<Box<dyn Object>>> =
		match runtime.run_with_parser(parser, program) {
			Ok(object) => object,
			Err(exception) => {
				eprintln!("{}", exception);
				return Ok(());
			}
		};

	if object.lock().unwrap().is::<Null>() {
	} else {
		println!("{}", object.lock().unwrap())
	}

	Ok(())
}
