// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

// use gl_bytecode::compiler::Compiler;
use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::parser::Parser;
use gl_core::source::Source;

pub fn run(filename: String) -> Result<(), AnyError> {
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

	let source: Source = Source::new_from_string(source).unwrap();
	let lexer: Lexer = Lexer::new(source, &format!("comp"));
	let mut parser: Parser = Parser::new(lexer);

	let _ = parser.run().ok().unwrap();

	// let mut bytecode = Compiler::from_ast("", ast.statements);
	// println!("{:?}", bytecode);

	Ok(())
}
