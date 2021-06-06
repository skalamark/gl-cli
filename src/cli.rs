// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use clap::{crate_version, Clap};

/// CLI GLanguage
#[derive(Clap)]
#[clap(bin_name = "gl", version = crate_version!())]
pub struct Opts {
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
	Repl(Repl),
	Eval(Eval),
	Run(Run),
	Fmt(Fmt),
}

/// Read Eval Print Loop
#[derive(Clap, Clone)]
pub struct Repl {}

/// Evaluate code in the shell
#[derive(Clap, Clone)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct Eval {
	/// Inspect interactively after running script
	#[clap(short, long)]
	pub inspect: bool,
	#[clap(setting = clap::ArgSettings::Required)]
	pub code_args: Vec<String>,
}

/// Run a program given a filename
#[derive(Clap, Clone)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct Run {
	/// Inspect interactively after running script
	#[clap(short, long)]
	pub inspect: bool,
	#[clap(setting = clap::ArgSettings::Required)]
	pub script_args: Vec<String>,
}

/// Format source files
#[derive(Clap, Clone)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct Fmt {
	#[clap(setting = clap::ArgSettings::Required)]
	pub filename: String,
	pub format_args: Vec<String>,
}
