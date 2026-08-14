use std::fmt::Display;
use std::time::Duration;

use crate::observer;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{BarChart, Block, Borders, Paragraph},
};

pub struct Animation<T> {
    data: Vec<T>,
    events: Vec<observer::SortEvent<T>>,
    position: usize,

    comparisons: usize,
    swaps: usize,
    writes: usize,
    comparing: Option<(usize, usize)>,
    swapping: Option<(usize, usize)>,
    counting: Option<(
        Vec<usize>,
        Vec<T>,
        crate::observer::CountingState<T>,
        Option<u32>,
    )>,
}

impl<T> Animation<T> {
    pub(crate) fn new(data: Vec<T>, events: Vec<observer::SortEvent<T>>) -> Animation<T> {
        Self {
            data,
            events,
            position: 0,
            comparisons: 0,
            swaps: 0,
            writes: 0,
            comparing: None,
            swapping: None,
            counting: None,
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
        self.counting = None;
        let event = self.events[self.position].clone();
        match event {
            observer::SortEvent::Swap { first, second } => {
                self.data.swap(first, second);
                self.swapping = Some((first, second));
                self.swaps += 1;
            }
            observer::SortEvent::Overwrite { index, value } => {
                self.data[index] = value;
                self.writes += 1;
            }
            observer::SortEvent::Compare { first, second } => {
                self.comparing = Some((first, second));
                self.comparisons += 1;
            }
            observer::SortEvent::Counting {
                counts,
                output,
                state,
                exp,
            } => {
                self.counting = Some((counts, output, state, exp));
                self.writes += 1;
            }
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

        if let Some((counts, output, state, exp)) = &animation.counting {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(area);

            // Input Array
            let input_labels: Vec<String> =
                (0..animation.data.len()).map(|i| i.to_string()).collect();
            let input_values: Vec<(&str, u64)> = animation
                .data
                .iter()
                .zip(input_labels.iter())
                .map(|(v, l)| (l.as_str(), v.to_string().parse::<u64>().unwrap_or(0)))
                .collect();
            let input_title = if let Some(e) = exp {
                format!("Input Array (exp: {})", e)
            } else {
                "Input Array".to_string()
            };
            let input_chart = BarChart::default()
                .block(Block::default().title(input_title).borders(Borders::ALL))
                .data(&input_values)
                .bar_width(3);
            frame.render_widget(input_chart, chunks[0]);

            // Counts Array
            let count_labels: Vec<String> = (0..counts.len()).map(|i| i.to_string()).collect();
            let count_values: Vec<(&str, u64)> = counts
                .iter()
                .zip(count_labels.iter())
                .map(|(&v, l)| (l.as_str(), v as u64))
                .collect();
            let count_chart = BarChart::default()
                .block(Block::default().title("Count Array").borders(Borders::ALL))
                .data(&count_values)
                .bar_width(3);
            frame.render_widget(count_chart, chunks[1]);

            // Output Array
            let output_labels: Vec<String> = (0..output.len()).map(|i| i.to_string()).collect();
            let output_values: Vec<(&str, u64)> = output
                .iter()
                .zip(output_labels.iter())
                .map(|(v, l)| (l.as_str(), v.to_string().parse::<u64>().unwrap_or(0)))
                .collect();
            let state_info;
            let state_str = match state {
                observer::CountingState::Counting => "Counting occurrences",
                observer::CountingState::Summing => "Cumulative sum",
                observer::CountingState::Placing { current_val } => {
                    state_info = format!("Placing {}", current_val);
                    &state_info
                }
            };
            let output_chart = BarChart::default()
                .block(
                    Block::default()
                        .title(format!("Output Array - {}", state_str))
                        .borders(Borders::ALL),
                )
                .data(&output_values)
                .bar_width(3);
            frame.render_widget(output_chart, chunks[2]);

            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

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
