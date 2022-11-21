use std::fs;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Frame;

pub struct StatefulList {
	pub selected: Option<usize>,
	pub root_dir: PathBuf,
	pub current_dir: PathBuf,
	pub state: ListState,
	pub items: Vec<PathBuf>,
}

impl StatefulList {
	pub fn new(root_dir: PathBuf) -> Self {
		Self {
			root_dir,
			selected: None,
			current_dir: PathBuf::new(),
			state: ListState::default(),
			items: Vec::new(),
		}
	}

	pub fn from_path(path: &Path) -> StatefulList {
		let mut list = Self::new(path.parent().unwrap().to_path_buf());
		list.change_directory(path);
		list
	}

	pub fn current_directory(&self) -> PathBuf {
		let mut path = self.root_dir.clone();
		path.push(&self.current_dir);
		path
	}

	pub fn first(&mut self) {
		self.state.select(Some(0));
	}

	pub fn last(&mut self) {
		self.state.select(Some(self.items.len() - 1));
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len() - 1 {
					0
				}
				else {
					i + 1
				}
			},
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len() - 1
				}
				else {
					i - 1
				}
			},
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn render<B>(&mut self, f: &mut Frame<B>, area: Rect)
	where
		B: Backend,
	{
		let items: Vec<ListItem> = self
			.items
			.iter()
			.enumerate()
			.map(|(i, path)| {
				let filename = path.file_name().unwrap().to_string_lossy().to_string();
				let span = Span::from(
					if path.is_dir() {
						format!("📁 {filename}")
					}
					else {
						format!("📄 {filename}")
					},
				);
				let mut style = Style::default();
				if path.is_dir() {
					style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
				}
				if filename.starts_with('.') {
					style = style.add_modifier(Modifier::DIM);
				}
				if Some(i) == self.selected {
					style = style.add_modifier(Modifier::REVERSED);
				}
				ListItem::new(span).style(style)
			})
			.collect();

		let current_dir = self.current_dir.to_string_lossy().to_string();
		let title = Spans::from(vec![
			Span::styled(
				"Files in ",
				Style::default()
					.add_modifier(Modifier::BOLD)
					.add_modifier(Modifier::UNDERLINED),
			),
			Span::styled(
				current_dir,
				Style::default()
					.fg(Color::Yellow)
					.add_modifier(Modifier::BOLD)
					.add_modifier(Modifier::UNDERLINED),
			),
		]);
		let items = List::new(items)
			.block(
				Block::default()
					.borders(Borders::ALL)
					.title(title)
					.title_alignment(Alignment::Center),
			)
			.highlight_style(Style::default().bg(Color::LightGreen).add_modifier(Modifier::BOLD));

		f.render_stateful_widget(items, area, &mut self.state);
	}

	pub fn go_down(&mut self) -> PathBuf {
		// Build selected path
		let selected = self.state.selected().unwrap();
		let selected = &self.items.get(selected).unwrap();
		let path = self.current_directory();
		let new_path = path.join(selected);

		// Update state
		if new_path.is_dir() {
			self.change_directory(&new_path)
		}
		else {
			self.select_file(&new_path)
		}
	}

	pub fn go_up(&mut self) -> PathBuf {
		// Build selected path
		let path = self.current_directory();
		let new_path = path.parent().unwrap().to_path_buf();

		// Check if new_path is the root
		if new_path == self.root_dir {
			return path;
		}

		// Update state
		self.change_directory(&new_path);

		// Return new path
		new_path
	}

	fn select_file(&mut self, path: &Path) -> PathBuf {
		if self.selected == self.state.selected() {
			self.selected = None;
			path.parent().unwrap().to_path_buf()
		}
		else {
			self.selected = self.state.selected();
			path.to_path_buf()
		}
	}

	pub fn change_directory(&mut self, path: &Path) -> PathBuf {
		// Update state
		let mut state = ListState::default();
		state.select(Some(0));
		self.state = state;

		// Update current directory
		self.current_dir = path.strip_prefix(&self.root_dir).unwrap().to_path_buf();

		// Clear selected files
		self.selected = None;

		// Update items
		self.items = fs::read_dir(path)
			.unwrap()
			.filter_map(std::result::Result::ok)
			.sorted_by(|d1, d2| {
				// Sort directories first
				let d1_is_dir = d1.path().is_dir();
				let d2_is_dir = d2.path().is_dir();
				if d1_is_dir && !d2_is_dir {
					std::cmp::Ordering::Less
				}
				else if !d1_is_dir && d2_is_dir {
					std::cmp::Ordering::Greater
				}
				else {
					// Sort by name
					d1.file_name().cmp(&d2.file_name())
				}
			})
			.map(|e| e.path())
			.collect();

		path.to_path_buf()
	}
}
