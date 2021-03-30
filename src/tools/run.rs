// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;
use gl_runtime::preludes::*;

pub fn run<T: Into<String>>(
	filename: T, module: T, program: &mut ProgramState,
) -> crate::ResultCli {
	let filename: String = filename.into();
	let module: String = module.into();
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

	let source: Source = Source::from_filename(filename).unwrap();
	let lexer: Lexer = Lexer::new(source, &module);
	let parser: Parser = Parser::new(lexer)?;
	let runtime: Runtime = Runtime::from_env(Rc::clone(&program.modules[&module]), module.clone());
	let _ = runtime.run_with_parser(parser)?;

	Ok(())
}
