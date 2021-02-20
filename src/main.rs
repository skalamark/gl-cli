// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::error::AnyError;
use gl_core::state::ProgramState;
use gl_std::Std;
use std::sync::{Arc, Mutex};

mod cli;
mod flags;
mod tools;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let flags: flags::Flags = match flags::Flags::from_args_vec(args) {
		Ok(flags) => flags,
		Err(err)
			if err.kind == clap::ErrorKind::HelpDisplayed
				|| err.kind == clap::ErrorKind::VersionDisplayed =>
		{
			err.write_to(&mut std::io::stdout()).unwrap();
			std::process::exit(0);
		}
		Err(err) => unwrap_or_exit(Err(AnyError::from(err))),
	};

	run_subcommand(flags).expect("");
}

fn unwrap_or_exit<T>(result: Result<T, AnyError>) -> T {
	match result {
		Ok(value) => value,
		Err(error) => {
			eprintln!("{}", format!("error: {}", error.to_string().trim()));
			std::process::exit(1);
		}
	}
}

fn run_subcommand(flags: flags::Flags) -> Result<(), AnyError> {
	use flags::GLanguageSubCommand;

	match flags.clone().subcommand {
		GLanguageSubCommand::Repl => run_repl(flags),
		GLanguageSubCommand::Eval { source } => run_eval(source, flags),
		GLanguageSubCommand::Run { filename, inspect } => run_run(filename, inspect, flags),
		GLanguageSubCommand::Compile { filename } => compile_run(filename, flags),
	}
}

fn run_repl(_: flags::Flags) -> Result<(), AnyError> {
	let env: gl_core::env::Env = Std::new();
	let mut program_state: ProgramState = ProgramState::new(Arc::new(Mutex::new(env)));
	program_state.env.crate_module = format!("repl");
	program_state.env.add_module(format!("repl"));
	let module: String = format!("repl");

	tools::repl::run(&module, &mut program_state, false)
}

fn run_eval(source: String, _: flags::Flags) -> Result<(), AnyError> {
	let env: gl_core::env::Env = Std::new();
	let mut program_state: ProgramState = ProgramState::new(Arc::new(Mutex::new(env)));
	program_state.env.crate_module = format!("eval");
	program_state.env.add_module(format!("eval"));
	let module: String = format!("eval");

	tools::eval::run(source, &module, &mut program_state)
}

fn run_run(filename: String, inspect: bool, _: flags::Flags) -> Result<(), AnyError> {
	let env: gl_core::env::Env = Std::new();
	let mut program_state: ProgramState = ProgramState::new(Arc::new(Mutex::new(env)));
	program_state.env.crate_module = format!("{}", &filename);
	program_state.env.add_module(format!("{}", &filename));
	let module: String = format!("{}", &filename);

	let r: Result<(), AnyError> = tools::run::run(filename, &module, &mut program_state);
	if r.is_err() {
		return Err(r.err().unwrap());
	}

	if inspect {
		tools::repl::run(&module, &mut program_state, true)
	} else {
		r
	}
}

fn compile_run(filename: String, _: flags::Flags) -> Result<(), AnyError> {
	let r: Result<(), AnyError> = tools::compile::run(filename);
	r
}
