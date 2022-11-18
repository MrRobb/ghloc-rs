#![allow(clippy::use_self)]

use clap::Parser;
use tokei::Languages;

mod cli;
mod git;
mod interface;

fn main() {
	// Get args
	let matches = cli::Args::parse();

	// Clone the repo
	let tempdir = tempfile::tempdir().unwrap();
	git::clone(&matches.url, tempdir.path());

	// Analyse the repo
	let config = tokei::Config::default();
	let mut languages = Languages::new();
	languages.get_statistics(&[tempdir.path()], &[], &config);

	// Display the results
	interface::display(tempdir, &languages);
}
