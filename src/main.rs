#![allow(clippy::use_self)]

use clap::Parser;

mod cli;
mod git;
mod interface;

fn main() {
	// Get args
	let matches = cli::Args::parse();

	// Create temporary directory
	let tempdir = tempfile::tempdir().unwrap();

	// Create directory with the name of the repo
	let repo_name = matches.url.as_str().split('/').last().unwrap();
	let repo_path = tempdir.path().join(repo_name);

	// Clone the repo
	git::clone(&matches.url, &repo_path);

	// Display the results
	interface::display(repo_path);
}
