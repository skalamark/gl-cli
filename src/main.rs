// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::error::AnyError;
use gl_core::state::ProgramState;

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

	run_subcommand(flags);
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
	}
}

fn run_repl(_: flags::Flags) -> Result<(), AnyError> {
	let mut program_state: ProgramState = ProgramState::new();
	let module: String = format!("repl");

	tools::repl::run(&module, &mut program_state)
}
