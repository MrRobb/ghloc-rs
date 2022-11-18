use std::path::{Path, PathBuf};

use tokei::Languages;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	text::Spans,
	widgets::{Block, Borders, List, ListItem},
	Frame,
};

pub struct CodeReport {
	pub report: Languages,
	pub root_dir: PathBuf,
	pub current_dir: PathBuf,
}

impl CodeReport {
	pub fn from_path(path: &Path) -> Self {
		let config = tokei::Config::default();
		let mut languages = Languages::new();
		languages.get_statistics(&[&path], &[], &config);
		let root_dir = path.parent().unwrap().to_path_buf();
		CodeReport {
			report: languages,
			current_dir: path.strip_prefix(&root_dir).unwrap().to_path_buf(),
			root_dir,
		}
	}

	pub fn change_path(&mut self, path: &Path) {
		let config = tokei::Config::default();
		let mut languages = Languages::new();
		languages.get_statistics(&[&path], &[], &config);
		self.report = languages;
		self.current_dir = path.strip_prefix(&self.root_dir).unwrap().into();
	}

	pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
		// Iterate through all elements in the `items` app and append some debug text to it.
		let items: Vec<ListItem> = self
			.report
			.iter()
			.map(|(lang_type, lang)| {
				let lines = vec![Spans::from(format!("{}: {}", lang_type, lang.code))];
				ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
			})
			.collect();

		// Create a List from all list items and highlight the currently selected one
		let items = List::new(items)
			.block(
				Block::default()
					.borders(Borders::ALL)
					.title(self.current_dir.to_string_lossy().to_string()),
			)
			.highlight_style(Style::default().bg(Color::LightGreen).add_modifier(Modifier::BOLD));

		// We can now render the item list
		f.render_widget(items, area);
	}
}
