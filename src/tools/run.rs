// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::parser::Parser;
use gl_core::source::Source;
use gl_core::state::ProgramState;
use gl_runtime::Runtime;
use std::sync::Arc;

pub fn run(filename: String, module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let path_filename: &std::path::Path = std::path::Path::new(&filename);
	if path_filename.exists() && path_filename.is_file() {
		if !filename.ends_with(".gl") {
			eprintln!("GL: Invalid file extension, expected file with extension '.gl'");
			return Ok(());
		}
	} else {
		eprintln!("GL: Can't open file '{}': No such file", &filename);
		return Ok(());
	};

	let source: Source = Source::new_from_filename(filename.clone()).unwrap();
	let lexer: Lexer = Lexer::new(source, module);
	let parser: Parser = Parser::new(lexer);
	let runtime: Runtime = Runtime::new_from_env(Arc::clone(&program.env.modules[module]), module);
	match runtime.run_with_parser(parser, program) {
		Ok(_) => {}
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	Ok(())
}
