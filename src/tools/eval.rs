// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::ast::AbstractSyntaxTree;
use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::object::Object;
use gl_core::parser::Parser;
use gl_core::state::ProgramState;
use gl_core::token::Token;
use gl_runtime::Runtime;

pub fn run(source: String, module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
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

	let runtime: Runtime = Runtime::new();
	let object: Object = match runtime.run(ast, module, program) {
		Ok(object) => object,
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	match object {
		Object::Null => {}
		o => println!("{}", o),
	}

	Ok(())
}
