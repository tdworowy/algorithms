use std::fmt::Display;
use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{BarChart, Block, Borders, Paragraph},
};

pub trait Renderer<T> {
    fn compare(&mut self, data: &[T], i: usize, j: usize);
    fn swap(&mut self, data: &[T], i: usize, j: usize);
    fn overwrite(&mut self, data: &[T], dst: usize, src: Option<usize>);
}

pub struct TerminalRenderer<T> {
    data: Vec<T>,
    active: Vec<usize>,

    comparisons: usize,
    swaps: usize,
    writes: usize,

    delay: Duration,
}

impl<T> TerminalRenderer<T>
where
    T: Clone + Display,
{
    pub fn new(delay_ms: u64) -> Self {
        Self {
            data: Vec::new(),
            active: Vec::new(),
            comparisons: 0,
            swaps: 0,
            writes: 0,
            delay: Duration::from_millis(delay_ms),
        }
    }

    pub fn set_data(&mut self, data: &[T]) {
        self.data = data.to_vec();
    }
    pub fn compare(&mut self, data: &[T], i: usize, j: usize) {
        self.data.clone_from_slice(data);

        self.active.clear();
        self.active.push(i);
        self.active.push(j);

        self.comparisons += 1;
    }
    pub fn swap(&mut self, data: &[T], i: usize, j: usize) {
        self.data.clone_from_slice(data);

        self.active.clear();
        self.active.push(i);
        self.active.push(j);

        self.swaps += 1;
    }
    pub fn overwrite(&mut self, data: &[T], dst: usize, src: Option<usize>) {
        self.data.clone_from_slice(data);

        self.active.clear();
        self.active.push(dst);

        if let Some(src) = src {
            self.active.push(src);
        }

        self.writes += 1;
    }
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

        let values: Vec<(&str, u64)> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, value)| {
                let label: &'static str = Box::leak(i.to_string().into_boxed_str());

                (label, value.to_string().parse::<u64>().unwrap_or(0))
            })
            .collect();

        let chart = BarChart::default()
            .block(Block::default().title("Sorting").borders(Borders::ALL))
            .data(&values)
            .bar_width(3)
            .bar_gap(1)
            .value_style(ratatui::style::Style::default());

        frame.render_widget(chart, chunks[0]);

        let stats = Paragraph::new(format!(
            "Comparisons: {}   Swaps: {}   Writes: {}",
            self.comparisons, self.swaps, self.writes,
        ))
        .block(Block::default().title("Statistics").borders(Borders::ALL));

        frame.render_widget(stats, chunks[1]);
    }
}
