use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::RngExt;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};

mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;

mod observer;
mod renderer;
use observer::TerminalVisualizationObserver;
use renderer::TerminalRenderer;

fn run_algorithm(choice: usize) {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut data: Vec<u32> = (0..20).map(|_| rng.random_range(1..70)).collect();

    let renderer = TerminalRenderer::new(100);

    let mut observer = TerminalVisualizationObserver::new(renderer);
    match choice {
        0 => quick_sort::quick_sort(&mut data, &mut observer),
        1 => bubble_sort::bubble_sort(&mut data, &mut observer),
        2 => insertion_sort::insertion_sort(&mut data, &mut observer),
        3 => heap_sort::heap_sort(&mut data, &mut observer),
        4 => merge_sort::merge_sort(&mut data, &mut observer),
        _ => {}
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let algorithms = vec![
        "Quick Sort",
        "Bubble Sort",
        "Insertion Sort",
        "Heap Sort",
        "Merge Sort",
        "Quit",
    ];

    let mut selected = 0;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let items: Vec<ListItem> = algorithms
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let prefix = if i == selected { "> " } else { "  " };

                    ListItem::new(format!("{}{}", prefix, name))
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title("Sorting Visualizer")
                    .borders(Borders::ALL),
            );

            frame.render_widget(list, area);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }
                KeyCode::Down => {
                    selected = (selected + 1) % algorithms.len();
                }
                KeyCode::Up => {
                    if selected == 0 {
                        selected = algorithms.len() - 1;
                    } else {
                        selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    if selected == algorithms.len() - 1 {
                        break;
                    }
                    run_algorithm(selected);
                }

                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

// TODO fix crash
// TODO switch BarChart to custom widget