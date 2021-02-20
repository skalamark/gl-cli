// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::crate_version;
use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::object::{Null, Object};
use gl_core::parser::Parser;
use gl_core::source::Source;
use gl_core::state::ProgramState;
use gl_runtime::Runtime;
use rustyline::{error::ReadlineError, Cmd, Editor, KeyEvent, Modifiers};
use std::sync::Arc;

pub fn run(module: &String, program: &mut ProgramState, inspect: bool) -> Result<(), AnyError> {
	let mut editor: Editor<()> = Editor::<()>::new();
	editor.load_history(".gl_history").unwrap_or(());

	editor.bind_sequence(KeyEvent::new('\r', Modifiers::NONE), Cmd::Newline);
	editor.bind_sequence(KeyEvent::alt('\r'), Cmd::AcceptLine);
	editor.bind_sequence(
		KeyEvent::new('\t', Modifiers::NONE),
		Cmd::Insert(1, format!("\t")),
	);

	if inspect == false {
		println!("GL {}", crate_version!());
		println!("exit using ctrl+d");
		println!("use alt+enter to run source");
	}

	loop {
		match editor.readline(">>> ") {
			Ok(source) => {
				editor.add_history_entry(source.clone());
				let source: Source = Source::new_from_string(source).unwrap();
				let lexer: Lexer = Lexer::new(source, module);
				let parser: Parser = Parser::new(lexer);
				let runtime: Runtime =
					Runtime::new_from_env(Arc::clone(&program.env.modules[module]), module);
				let object: std::sync::Arc<std::sync::Mutex<Box<dyn Object>>> =
					match runtime.run_with_parser(parser, program) {
						Ok(object) => object,
						Err(exception) => {
							eprintln!("{}", exception);
							continue;
						}
					};
				editor.save_history(".gl_history").unwrap_or(());

				if object.lock().unwrap().is::<Null>() {
				} else {
					println!("{}", object.lock().unwrap())
				}
			}
			Err(ReadlineError::Interrupted) => {
				println!("exit using ctrl+d");
				continue;
			}
			Err(ReadlineError::Eof) => {
				break;
			}
			Err(err) => {
				eprintln!("Error: {:?}", err);
				break;
			}
		}
	}

	Ok(())
}
