use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tempfile::TempDir;
use tokei::Languages;
use tui::backend::{Backend, CrosstermBackend};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_tree_widget::Tree;

use self::file_tree::StatefulTree;

mod file_tree;

struct App<'a> {
	file_tree: StatefulTree<'a>,
}

impl<'a> App<'a> {
	fn new(tempdir: TempDir) -> Self {
		App {
			file_tree: StatefulTree::from_path(tempdir.into_path()),
		}
	}
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
	loop {
		terminal.draw(|f| {
			let area = f.size();

			let items = Tree::new(app.file_tree.items.clone())
				.block(
					Block::default()
						.borders(Borders::ALL)
						.title(format!("Tree Widget {:?}", app.file_tree.state)),
				)
				.highlight_style(
					Style::default()
						.fg(Color::Black)
						.bg(Color::LightGreen)
						.add_modifier(Modifier::BOLD),
				)
				.highlight_symbol(">> ");
			f.render_stateful_widget(items, area, &mut app.file_tree.state);
		})?;

		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char('q') => return Ok(()),
				KeyCode::Char('\n' | ' ') => app.file_tree.toggle(),
				KeyCode::Left => app.file_tree.left(),
				KeyCode::Right => app.file_tree.right(),
				KeyCode::Down => app.file_tree.down(),
				KeyCode::Up => app.file_tree.up(),
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
