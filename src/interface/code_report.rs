use std::path::{Path, PathBuf};

use itertools::Itertools;
use tokei::Languages;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BarChart, Block, Borders};
use tui::Frame;

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
		let current_dir = self.current_dir.to_string_lossy().to_string();
		let style = Style::default()
			.add_modifier(Modifier::BOLD)
			.add_modifier(Modifier::UNDERLINED);
		let title = Spans::from(vec![
			Span::styled("Lines of code in ", style),
			Span::styled(current_dir, style.fg(Color::Yellow)),
		]);

		let data = self
			.report
			.iter()
			.filter(|(_, lang)| lang.code > 0)
			.sorted_by(|(_, lang1), (_, lang2)| lang1.code.cmp(&lang2.code).reverse())
			.map(|(lang_type, lang)| (lang_type.to_string(), lang.code as u64))
			.collect::<Vec<_>>();
		let binding = data
			.iter()
			.map(|(lang_type, loc)| (lang_type.as_str(), *loc))
			.collect::<Vec<_>>();
		let max_label_size = data.iter().map(|(lang_type, _)| lang_type.len()).max().unwrap_or(0);
		let barchart = BarChart::default()
			.block(
				Block::default()
					.borders(Borders::ALL)
					.title(title)
					.title_alignment(Alignment::Center),
			)
			.data(binding.as_slice())
			.bar_width(u16::try_from(max_label_size).unwrap_or(5))
			.bar_gap(3)
			.bar_style(Style::default().fg(Color::Yellow))
			.value_style(
				Style::default()
					.fg(Color::Black)
					.bg(Color::Yellow)
					.add_modifier(Modifier::BOLD),
			);
		f.render_widget(barchart, area);
	}
}
