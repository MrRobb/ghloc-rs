use std::{fs, path::PathBuf};

use itertools::Itertools;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	text::Spans,
	widgets::{Block, Borders, List, ListItem, ListState},
	Frame,
};

pub struct StatefulList<T>
where
	T: Clone,
	Spans<'static>: From<T>,
{
	pub state: ListState,
	pub items: Vec<T>,
}

impl<T> StatefulList<T>
where
	T: Clone,
	Spans<'static>: From<T>,
{
	#[allow(dead_code)]
	pub fn with_items(items: Vec<T>) -> StatefulList<T> {
		StatefulList {
			state: ListState::default(),
			items,
		}
	}

	pub fn from_path(path: PathBuf) -> StatefulList<String> {
		let mut state = ListState::default();
		state.select(Some(0));
		StatefulList::<String> {
			state,
			items: fs::read_dir(path)
				.unwrap()
				.filter_map(std::result::Result::ok)
				.sorted_by(|d1, d2| {
					// Sort directories first
					let d1_is_dir = d1.path().is_dir();
					let d2_is_dir = d2.path().is_dir();
					if d1_is_dir && !d2_is_dir {
						std::cmp::Ordering::Less
					} else if !d1_is_dir && d2_is_dir {
						std::cmp::Ordering::Greater
					} else {
						// Sort by name
						d1.file_name().cmp(&d2.file_name())
					}
				})
				.map(|e| e.path().file_name().unwrap().to_string_lossy().to_string())
				.collect(),
		}
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
				} else {
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
				} else {
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
		// Iterate through all elements in the `items` app and append some debug text to it.
		let items: Vec<ListItem> = self
			.items
			.iter()
			.map(|i| {
				let lines = vec![Spans::from(i.clone())];
				ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
			})
			.collect();

		// Create a List from all list items and highlight the currently selected one
		let items = List::new(items)
			.block(Block::default().borders(Borders::ALL).title("List"))
			.highlight_style(Style::default().bg(Color::LightGreen).add_modifier(Modifier::BOLD))
			.highlight_symbol(">> ");

		// We can now render the item list
		f.render_stateful_widget(items, area, &mut self.state);
	}
}
