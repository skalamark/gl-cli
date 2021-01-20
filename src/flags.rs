// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::cli::create_app;
use clap::{App, ArgMatches};

#[derive(Clone, Debug)]
pub enum GLanguageSubCommand {
	Repl,
	Eval { source: String },
}

impl Default for GLanguageSubCommand {
	fn default() -> Self {
		Self::Repl
	}
}

#[derive(Clone, Debug, Default)]
pub struct Flags {
	pub argv: Vec<String>,
	pub subcommand: GLanguageSubCommand,
	pub repl: bool,
}

impl Flags {
	pub fn from_args_vec(args: Vec<String>) -> clap::Result<Flags> {
		let app: App = create_app();
		let matches: ArgMatches = app.get_matches_from_safe(args).map_err(|e| clap::Error {
			message: e.message.trim_start_matches("error: ").to_string(),
			..e
		})?;

		let mut flags: Flags = Self::default();

		if let Some(m) = matches.subcommand_matches("repl") {
			repl_parse(&mut flags, m);
		} else if let Some(m) = matches.subcommand_matches("eval") {
			eval_parse(&mut flags, m);
		} else {
			repl_parse(&mut flags, &matches);
		}

		Ok(flags)
	}
}

fn repl_parse(flags: &mut Flags, _: &clap::ArgMatches) {
	flags.repl = true;
	flags.subcommand = GLanguageSubCommand::Repl;
}

fn eval_parse(flags: &mut Flags, matches: &clap::ArgMatches) {
	flags.repl = true;
	let source = matches.value_of("source").unwrap().to_string();
	flags.subcommand = GLanguageSubCommand::Eval { source };
}
