// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

#[derive(Clone, Debug)]
pub enum GLanguageSubCommand {
	Repl,
}

impl Default for GLanguageSubCommand {
	fn default() -> Self {
		Self::Repl
	}
}
