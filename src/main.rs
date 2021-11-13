// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::Parser;
use cli::*;
use gl_core::preludes::*;
use gl_std::*;
use starts::{start_eval, start_fmt, start_repl};

use crate::starts::start_run;

mod cli;
mod starts;

type ResultCli = Result<(), Exception>;

fn main() {
	let opts: Opts = match Opts::try_parse() {
		Ok(opts) => opts,
		Err(err) if err.kind == clap::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand =>
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

fn create_program() -> Interpreter {
	let mut interpreter: Interpreter = Interpreter::new();
	Std::new(&mut interpreter);
	interpreter
}

fn run_repl() -> ResultCli {
	let mut program = create_program();
	start_repl(&mut program)
}

fn run_eval(source: String, inspect: bool, _: Vec<String>) -> ResultCli {
	let mut program = create_program();
	let result_program = start_eval(&source, &mut program);

	if inspect {
		if let Err(exception) = result_program {
			eprintln!("{}", exception)
		}

		println!();
		return start_repl(&mut program);
	}

	result_program
}

fn run_run(filename: String, inspect: bool, _: Vec<String>) -> ResultCli {
	let mut program = create_program();
	let result_program = start_run(&filename, &mut program);

	if inspect {
		if let Err(exception) = result_program {
			eprintln!("{}", exception)
		}

		println!();
		return start_repl(&mut program);
	}

	result_program
}

fn fmt_run(filename: String, _: Vec<String>) -> ResultCli { start_fmt(&filename) }
