// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::ast::AbstractSyntaxTree;
use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::parser::Parser;
use gl_core::state::ProgramState;
use gl_core::token::Token;
use gl_runtime::Runtime;
use std::rc::Rc;

pub fn run(filename: String, module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let source: String = {
		let path_filename: &std::path::Path = std::path::Path::new(&filename);
		if path_filename.exists() && path_filename.is_file() {
			if filename.ends_with(".gl") {
				std::fs::read_to_string(&filename).expect("")
			} else {
				eprintln!("GL: Invalid file extension, expected file with extension '.gl'");
				return Ok(());
			}
		} else {
			eprintln!("GL: Can't open file '{}': No such file", &filename);
			return Ok(());
		}
	};

	let mut lexer: Lexer = Lexer::new();
	let tokens: Vec<Token> = match lexer.run(source, module, program) {
		Ok(tokens) => tokens,
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	let mut parser: Parser = Parser::new();
	let ast: AbstractSyntaxTree = match parser.run(tokens, module, program) {
		Ok(ast) => ast,
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	let runtime: Runtime = Runtime::new_from_env(Rc::clone(&program.env.modules[module]));
	match runtime.run(ast, module, program) {
		Ok(_) => {}
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	Ok(())
}
