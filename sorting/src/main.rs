use crate::renderer::TerminalRenderer;
use crossterm::event::KeyEventKind;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem},
};
use std::io;
use std::time::Duration;

mod bubble_sort;
mod heap_sort;
mod insertion_and_merge_sort;
mod insertion_sort;
mod merge_sort;
mod observer;
mod quick_sort;
mod renderer;
mod selection_sort;
mod sort_runner;

fn create_algorithm(choice: usize) -> sort_runner::SortRunner {
    sort_runner::SortRunner::new(choice)
}
enum Mode {
    Menu,
    Visualizing,
}
struct App {
    algorithms: Vec<&'static str>,
    selected: usize,
    renderer: TerminalRenderer<u32>,
    mode: Mode,
}
impl App {
    fn new() -> Self {
        Self {
            algorithms: vec![
                "Quick Sort",
                "Bubble Sort",
                "Insertion Sort",
                "Selection Sort",
                "Heap Sort",
                "Merge Sort",
                "Quit",
            ],
            selected: 0,
            renderer: TerminalRenderer::new(100),
            mode: Mode::Menu,
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut runner: Option<sort_runner::SortRunner> = None;

    loop {
        if let Some(runner) = &mut runner {
            if !runner.finished() {
                runner.step();
            }
        }
        terminal.draw(|frame| match app.mode {
            Mode::Menu => {
                let items: Vec<ListItem> = app
                    .algorithms
                    .iter()
                    .enumerate()
                    .map(|(i, name)| {
                        let prefix = if i == app.selected { "> " } else { "  " };

                        ListItem::new(format!("{}{}", prefix, name))
                    })
                    .collect();

                let list = List::new(items).block(
                    Block::default()
                        .title("Sorting Visualizer")
                        .borders(Borders::ALL),
                );

                frame.render_widget(list, frame.area());
            }

            Mode::Visualizing => {
                if let Some(runner) = &runner {
                    app.renderer.draw(frame, &runner.animation);
                }
            }
        })?;

        if event::poll(Duration::from_millis(30))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break;
                    }

                    KeyCode::Down => {
                        if let Mode::Menu = app.mode {
                            app.selected = (app.selected + 1) % app.algorithms.len();
                        }
                    }
                    KeyCode::Up => {
                        if let Mode::Menu = app.mode {
                            if app.selected == 0 {
                                app.selected = app.algorithms.len() - 1;
                            } else {
                                app.selected -= 1;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Mode::Menu = app.mode {
                            if app.selected == app.algorithms.len() - 1 {
                                break;
                            }

                            runner = Some(create_algorithm(app.selected));

                            app.mode = Mode::Visualizing;
                        }
                    }
                    KeyCode::Char('r') => {
                        runner = None;
                        app.mode = Mode::Menu;
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
