use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tempfile::TempDir;
use tokei::Languages;
use tui::Terminal;
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	Frame,
};

use self::file_viewer::StatefulList;

mod file_tree;
mod file_viewer;

pub struct App {
	pub current_dir: TempDir,
	pub file_tree: StatefulList<String>,
}

impl App {
	fn new(tempdir: TempDir) -> Self {
		App {
			file_tree: StatefulList::<String>::from_path(tempdir.path().to_path_buf()),
			current_dir: tempdir,
		}
	}
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
	// Create two chunks with equal horizontal screen space
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
		.split(f.size());

	app.file_tree.render(f, chunks[0]);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
	loop {
		terminal.draw(|f| ui(f, &mut app))?;

		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char('q') => return Ok(()),
				KeyCode::Down => app.file_tree.next(),
				KeyCode::Up => app.file_tree.previous(),
				KeyCode::Home => app.file_tree.first(),
				KeyCode::End => app.file_tree.last(),
				_ => {},
			}
		}
	}
}

pub fn display(tempdir: TempDir, _report: &Languages) {
	// Terminal initialization
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend).unwrap();

	// App
	let app = App::new(tempdir);
	let res = run_app(&mut terminal, app);

	// restore terminal
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
	terminal.show_cursor().unwrap();

	if let Err(err) = res {
		println!("{:?}", err);
	}
}
