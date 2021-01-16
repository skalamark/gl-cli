// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::crate_version;
use gl_core::ast::AbstractSyntaxTree;
use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::object::Object;
use gl_core::parser::Parser;
use gl_core::state::ProgramState;
use gl_core::token::Token;
use gl_runtime::Runtime;
use rustyline::{error::ReadlineError, Cmd, Editor, KeyEvent, Modifiers};

pub fn run(module: &String, program: &mut ProgramState) -> Result<(), AnyError> {
	let mut editor: Editor<()> = Editor::<()>::new();
	editor.load_history(".gl_history").unwrap_or(());

	editor.bind_sequence(KeyEvent::new('\r', Modifiers::NONE), Cmd::Newline);
	editor.bind_sequence(KeyEvent::alt('\r'), Cmd::AcceptLine);

	println!("GL {}", crate_version!());
	println!("exit using ctrl+d");
	println!("use alt+enter to run source");

	loop {
		match editor.readline(">>> ") {
			Ok(source) => {
				let mut lexer: Lexer = Lexer::new();
				let tokens: Vec<Token> = match lexer.run(source.clone(), module, program) {
					Ok(tokens) => tokens,
					Err(exception) => {
						eprintln!("{}", exception);
						continue;
					}
				};
				editor.add_history_entry(source);

				let mut parser: Parser = Parser::new();
				let ast: AbstractSyntaxTree = match parser.run(tokens, module, program) {
					Ok(ast) => ast,
					Err(exception) => {
						eprintln!("{}", exception);
						continue;
					}
				};

				let runtime: Runtime = Runtime::new();
				let object: Object = match runtime.run(ast, module, program) {
					Ok(ast) => ast,
					Err(exception) => {
						eprintln!("{}", exception);
						continue;
					}
				};

				match object {
					Object::Null => {}
					o => println!("{}", o),
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

	editor.save_history(".gl_history").unwrap_or(());

	Ok(())
}
