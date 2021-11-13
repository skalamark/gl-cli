// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::io::Write;

use clap::crate_version;
use gl_core::preludes::*;
use gl_fmt::Format;
use rustyline::error::ReadlineError;
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};
use termcolor::Color::{Green, Red, Yellow};
use termcolor::{Ansi, ColorSpec, WriteColor};

fn style(s: &str, colorspec: ColorSpec) -> String {
	let mut v = Vec::new();
	let mut ansi_writer = Ansi::new(&mut v);
	ansi_writer.set_color(&colorspec).unwrap();
	ansi_writer.write_all(s.as_bytes()).unwrap();
	ansi_writer.reset().unwrap();
	format!("{}", String::from_utf8_lossy(&v))
}

pub fn green(s: &str) -> String {
	let mut style_spec = ColorSpec::new();
	style_spec.set_fg(Some(Green));
	style(&s, style_spec)
}

pub fn red(s: &str) -> String {
	let mut style_spec = ColorSpec::new();
	style_spec.set_fg(Some(Red));
	style(&s, style_spec)
}

pub fn yellow(s: &str) -> String {
	let mut style_spec = ColorSpec::new();
	style_spec.set_fg(Some(Yellow));
	style(&s, style_spec)
}

#[derive(Helper, Highlighter, Completer, Hinter)]
struct MyHelper {
	validator: MatchingBracketValidator,
}

impl Validator for MyHelper {
	fn validate(
		&self, ctx: &mut validate::ValidationContext,
	) -> rustyline::Result<validate::ValidationResult> {
		self.validator.validate(ctx)
	}

	fn validate_while_typing(&self) -> bool { self.validator.validate_while_typing() }
}

// REPL

pub fn start_repl(interpreter: &mut Interpreter) -> crate::ResultCli {
	let config: Config = Config::builder()
		.history_ignore_space(true)
		.completion_type(CompletionType::Circular)
		.edit_mode(EditMode::Vi)
		.auto_add_history(true)
		.tab_stop(4)
		.indent_size(4)
		.build();
	let mut rl = Editor::with_config(config);

	rl.set_helper(Some(MyHelper { validator: MatchingBracketValidator::new() }));
	rl.load_history(".gl_history").unwrap_or(());
	rl.bind_sequence(KeyEvent::new('\t', Modifiers::NONE), Cmd::Insert(1, format!("\t")));

	println!("GL {}", crate_version!());
	println!("exit using ctrl+d or close()");

	let mut prompt_count = 0;

	loop {
		let prompt = green(&format!("[{}]-> ", prompt_count));
		prompt_count += 1;

		match rl.readline(&prompt) {
			Ok(source) => {
				let object: Object = match interpreter.eval(source) {
					Ok(object) => object,
					Err(exception) => {
						eprintln!("{}", red(&exception.to_string()));
						continue;
					},
				};

				match object {
					Object::Null => {},
					object => println!("{}", object),
				}
			},
			Err(ReadlineError::Interrupted) => {
				println!("{}", yellow("exit using ctrl+d or close()"));
				continue;
			},
			Err(ReadlineError::Eof) => break,
			Err(err) => break eprintln!("Error: {}", err),
		}
	}

	rl.save_history(".gl_history").unwrap_or(());
	Ok(())
}

// FMT

pub fn start_fmt<T: Into<String>>(filename: T) -> crate::ResultCli {
	let filename: String = filename.into();
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

	let source: Source = Source::from_filename(filename.clone()).unwrap();
	let lexer: Lexer = Lexer::new(source, &filename);
	let parser: Parser = Parser::new(lexer)?;
	let mut format: Format = Format::new();
	let source_formated = format.run_with_parser(parser)?;
	std::fs::write(filename, source_formated.as_bytes()).unwrap();

	Ok(())
}

// EVAL

pub fn start_eval<T: Into<String>>(source: T, interpreter: &mut Interpreter) -> crate::ResultCli {
	match interpreter.eval(source)? {
		Object::Null => {},
		object => println!("{}", object),
	}

	Ok(())
}

// RUN

pub fn start_run<T: Into<String>>(filename: T, interpreter: &mut Interpreter) -> crate::ResultCli {
	let filename: String = filename.into();
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

	interpreter.script(filename)?;

	Ok(())
}
