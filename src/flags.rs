// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::cli::create_app;
use clap::{App, ArgMatches};

#[derive(Clone, Debug)]
pub enum GLanguageSubCommand {
	Repl,
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
		} else {
			repl_parse(&mut flags, &matches);
		}

		Ok(flags)
	}
}
