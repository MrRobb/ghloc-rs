use url::Url;

#[derive(clap::Parser)]
#[clap(about, version, author)]
pub struct Args {
	/// Url to the git repository
	pub url: Url,
}
