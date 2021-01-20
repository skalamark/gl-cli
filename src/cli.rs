// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::crate_version;
use clap::{App, Arg, SubCommand};

const USAGE: &str = "
gl [SUBCOMMAND]

To start the REPL:
    gl
";

pub fn create_app<'a, 'b>() -> App<'a, 'b> {
	clap::App::new("gl")
		.bin_name("gl")
		.version(crate_version!())
		.long_version(crate_version!())
		.about("Interface de linha de comando para utilização da linguagem de script GLanguage")
		.usage(USAGE.trim())
		.subcommand(repl_subcommand())
		.subcommand(eval_subcommand())
		.subcommand(run_subcommand())
}

fn repl_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("repl").about("Read Eval Print Loop")
}

fn eval_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("eval")
		.about("Evaluate source from the command line")
		.arg(Arg::with_name("source").required(true))
}

fn run_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("run")
		.about("Run program from a script file")
		.arg(Arg::with_name("filename").required(true))
}
