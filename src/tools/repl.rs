// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::crate_version;
use gl_core::preludes::*;
use gl_core::state::ProgramState;
use gl_runtime::preludes::*;
use rustyline::{error::ReadlineError, Cmd, Editor, KeyEvent, Modifiers};

pub fn run(module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let mut editor: Editor<()> = Editor::<()>::new();
	editor.load_history(".gl_history").unwrap_or(());

	editor.bind_sequence(KeyEvent::new('\r', Modifiers::NONE), Cmd::Newline);
	editor.bind_sequence(KeyEvent::alt('\r'), Cmd::AcceptLine);
	editor.bind_sequence(
		KeyEvent::new('\t', Modifiers::NONE),
		Cmd::Insert(1, format!("\t")),
	);

	println!("GL {}", crate_version!());
	println!("exit using ctrl+d");
	println!("use alt+enter to run source");

	loop {
		match editor.readline(">>> ") {
			Ok(source) => {
				editor.add_history_entry(source.clone());
				editor.save_history(".gl_history").unwrap_or(());
				let source: Source = Source::from_string(source).unwrap();
				let lexer: Lexer = Lexer::new(source, module);
				let parser: Parser = Parser::new(lexer);
				let runtime: Runtime =
					Runtime::from_env(Rc::clone(&program.env.modules[module]), module.clone());
				let object: Object = match runtime.run_with_parser(parser) {
					Ok(object) => object,
					Err(exception) => {
						eprintln!("{}", exception);
						continue;
					}
				};

				match object {
					Object::Null => {}
					object => println!("{}", object),
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
