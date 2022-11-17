use std::path::PathBuf;

use itertools::Itertools;
use tui_tree_widget::{TreeItem, TreeState};

pub struct StatefulTree<'a> {
	pub state: TreeState,
	pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
	#[allow(dead_code)]
	pub fn new() -> Self {
		Self {
			state: TreeState::default(),
			items: Vec::new(),
		}
	}

	#[allow(dead_code)]
	pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
		Self {
			state: TreeState::default(),
			items,
		}
	}

	pub fn from_path(path: PathBuf) -> Self {
		Self {
			state: TreeState::default(),
			items: Self::get_tree_from_path(path),
		}
	}

	/// Add recursively all files in a directory to a tree
	fn get_tree_from_path(path: PathBuf) -> Vec<TreeItem<'a>> {
		std::fs::read_dir(path)
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
			.map(|e| {
				let path = e.path();
				let file_name = path.file_name().unwrap().to_string_lossy().to_string();
				if path.is_dir() {
					TreeItem::new(file_name, Self::get_tree_from_path(e.path()))
				} else {
					TreeItem::new_leaf(file_name)
				}
			})
			.collect()
	}

	pub fn first(&mut self) {
		self.state.select_first();
	}

	pub fn last(&mut self) {
		self.state.select_last(&self.items);
	}

	pub fn down(&mut self) {
		self.state.key_down(&self.items);
	}

	pub fn up(&mut self) {
		self.state.key_up(&self.items);
	}

	pub fn left(&mut self) {
		self.state.key_left();
	}

	pub fn right(&mut self) {
		self.state.key_right();
	}

	pub fn toggle(&mut self) {
		self.state.toggle_selected();
	}
}
