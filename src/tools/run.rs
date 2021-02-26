// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;
use gl_core::state::ProgramState;
use gl_runtime::preludes::*;

pub fn run(filename: String, module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let source: Source = {
		let path_filename: &std::path::Path = std::path::Path::new(&filename);
		if path_filename.exists() && path_filename.is_file() {
			if filename.ends_with(".gl") {
				Source::from_string(std::fs::read_to_string(&filename).expect("")).unwrap()
			} else {
				eprintln!("GL: Invalid file extension, expected file with extension '.gl'");
				return Ok(());
			}
		} else {
			eprintln!("GL: Can't open file '{}': No such file", &filename);
			return Ok(());
		}
	};

	let lexer: Lexer = Lexer::new(source, module);
	let parser: Parser = Parser::new(lexer);
	let runtime: Runtime =
		Runtime::from_env(Rc::clone(&program.env.modules[module]), module.clone());
	match runtime.run_with_parser(parser) {
		Ok(_) => {}
		Err(exception) => {
			eprintln!("{}", exception);
			return Ok(());
		}
	};

	Ok(())
}
