use std::fmt::Display;
use std::time::Duration;

use crate::observer::SortEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{BarChart, Block, Borders, Paragraph},
};

pub struct Animation<T> {
    data: Vec<T>,
    events: Vec<SortEvent>,
    position: usize,

    comparisons: usize,
    swaps: usize,
    writes: usize,
    comparing: Option<(usize, usize)>,
    swapping: Option<(usize, usize)>,
}

impl<T> Animation<T> {
    pub(crate) fn new(data: Vec<T>, events: Vec<SortEvent>) -> Animation<T> {
        Self {
            data,
            events,
            position: 0,
            comparisons: 0,
            swaps: 0,
            writes: 0,
            comparing: None,
            swapping: None,
        }
    }
    pub fn data(&self) -> &[T] {
        &self.data
    }
    pub fn comparing(&self) -> Option<(usize, usize)> {
        self.comparing
    }
    pub fn swapping(&self) -> Option<(usize, usize)> {
        self.swapping
    }
    pub fn comparisons(&self) -> usize {
        self.comparisons
    }
    pub fn swaps(&self) -> usize {
        self.swaps
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<T: Clone> Animation<T> {
    pub fn step(&mut self) {
        if self.finished() {
            return;
        }
        self.comparing = None;
        self.swapping = None;
        let event = &self.events[self.position];
        match event {
            SortEvent::Swap { first, second } => {
                self.data.swap(*first, *second);
                self.swapping = Some((*first, *second));
                self.swaps += 1;
            }
            SortEvent::Overwrite { index, source } => {
                if let Some(source) = source {
                    self.data[*index] = self.data[*source].clone();
                    self.writes += 1;
                }
            }
            SortEvent::Compare { first, second } => {
                self.comparing = Some((*first, *second));
                self.comparisons += 1;
            }
            _ => {}
        }
        self.position += 1;
    }
    pub fn finished(&self) -> bool {
        self.position >= self.events.len()
    }
}

pub struct TerminalRenderer<T> {
    data: Vec<T>,
    active: Vec<usize>,
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
            delay: Duration::from_millis(delay_ms),
        }
    }

    pub fn set_data(&mut self, data: &[T]) {
        self.data = data.to_vec();
    }
    pub fn draw(&self, frame: &mut Frame, animation: &Animation<T>)
    where
        T: Display,
    {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

        // Keep labels alive during rendering
        let labels: Vec<String> = animation
            .data
            .iter()
            .enumerate()
            .map(|(i, _)| i.to_string())
            .collect();

        let values: Vec<(&str, u64)> = animation
            .data
            .iter()
            .zip(labels.iter())
            .map(|(value, label)| {
                (
                    label.as_str(),
                    value.to_string().parse::<u64>().unwrap_or(0),
                )
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
            animation.comparisons, animation.swaps, animation.writes,
        ))
        .block(Block::default().title("Statistics").borders(Borders::ALL));

        frame.render_widget(stats, chunks[1]);
    }
}
