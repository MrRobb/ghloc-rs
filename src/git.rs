use git2::{
	build::{CheckoutBuilder, RepoBuilder},
	FetchOptions, RemoteCallbacks,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::{cell::RefCell, path::Path};
use url::Url;

pub fn clone(repo: &Url, path: &Path) {
	let pb = RefCell::new(
		ProgressBar::new(0).with_style(
			ProgressStyle::default_bar()
				.template("{spinner:.green} [{elapsed_precise}] {msg} [{wide_bar:.cyan/blue}] {pos}/{total} ({eta})")
				.unwrap()
				.progress_chars("=> "),
		),
	);

	let mut cb = RemoteCallbacks::new();
	cb.transfer_progress(|stats| {
		let pb = pb.borrow_mut();
		pb.set_message("Fetching...");
		pb.set_length(stats.total_objects() as u64);
		pb.set_position(stats.received_objects() as u64);
		true
	});

	let mut co = CheckoutBuilder::new();
	co.progress(|path, cur, total| {
		let pb = pb.borrow_mut();
		pb.set_message(
			path.map(std::path::Path::to_string_lossy)
				.unwrap_or_default()
				.to_string(),
		);
		pb.set_length(total as u64);
		pb.set_position(cur as u64);
	});

	let mut fo = FetchOptions::new();
	fo.remote_callbacks(cb);
	RepoBuilder::new()
		.fetch_options(fo)
		.with_checkout(co)
		.clone(repo.as_str(), Path::new(path))
		.unwrap();
}
