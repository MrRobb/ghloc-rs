use std::{io, path::Path};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::Terminal;
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	Frame,
};

use self::{code_report::CodeReport, file_viewer::StatefulList};

mod code_report;
mod file_viewer;

pub struct App {
	pub file_tree: StatefulList,
	pub code_report: CodeReport,
}

impl App {
	fn new(path: &Path) -> Self {
		App {
			code_report: CodeReport::from_path(path),
			file_tree: StatefulList::from_path(path),
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
	app.code_report.render(f, chunks[1]);
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
				KeyCode::Enter => {
					let new_dir = app.file_tree.go_down();
					app.code_report.change_path(&new_dir);
				},
				KeyCode::Backspace | KeyCode::Esc | KeyCode::Left => {
					let new_dir = app.file_tree.go_up();
					app.code_report.change_path(&new_dir);
				},
				_ => {},
			}
		}
	}
}

pub fn display(tempdir: &Path) {
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
