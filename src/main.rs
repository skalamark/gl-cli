// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::Clap;
use cli::*;
use gl_core::preludes::*;
use gl_std::*;

mod cli;
mod tools;

type ResultCli = Result<(), Exception>;

fn main() {
	let opts: Opts = match Opts::try_parse() {
		Ok(opts) => opts,
		Err(err) if err.kind == clap::ErrorKind::MissingArgumentOrSubcommand =>
			Opts { subcmd: SubCommand::Repl(Repl {}) },
		Err(err) => {
			err.exit();
		},
	};

	match {
		match opts.subcmd {
			SubCommand::Repl(Repl {}) => run_repl(),
			SubCommand::Eval(Eval { inspect, mut code_args }) => {
				let source = code_args.remove(0);
				run_eval(source, inspect, code_args)
			},
			SubCommand::Run(Run { inspect, mut script_args }) => {
				let filename = script_args.remove(0);
				run_run(filename, inspect, script_args)
			},
			SubCommand::Fmt(Fmt { filename, format_args }) => fmt_run(filename, format_args),
		}
	} {
		Ok(_) => {},
		Err(exception) => {
			eprintln!("{}", exception)
		},
	}
}

fn create_program<T: Into<String>>(module: T) -> (ProgramState, String) {
	let module: String = module.into();
	let mut program_state: ProgramState = ProgramState::with_std(&module, Std::new());
	program_state.add_module(&module);
	(program_state, module)
}

fn run_repl() -> ResultCli {
	let (mut program, module) = create_program("repl");
	tools::repl::run(module, &mut program)
}

fn run_eval(source: String, inspect: bool, _: Vec<String>) -> ResultCli {
	let (mut program, module) = create_program("eval");
	let result_program = tools::eval::run(&source, &module, &mut program);

	if inspect {
		if let Err(exception) = result_program {
			eprintln!("{}", exception)
		}

		println!();
		return tools::repl::run(module, &mut program);
	}

	result_program
}

fn run_run(filename: String, inspect: bool, _: Vec<String>) -> ResultCli {
	let (mut program, module) = create_program(&filename);
	let result_program = tools::run::run(&filename, &module, &mut program);

	if inspect {
		if let Err(exception) = result_program {
			eprintln!("{}", exception)
		}

		println!();
		return tools::repl::run(module, &mut program);
	}

	result_program
}

fn fmt_run(filename: String, _: Vec<String>) -> ResultCli {
	let (_, module) = create_program(&filename);
	let result_program = tools::fmt::run(&filename, &module);
	result_program
}
