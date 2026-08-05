pub trait Renderer<T> {
    fn compare(&mut self, data: &[T], i: usize, j: usize);
    fn swap(&mut self, data: &[T], i: usize, j: usize);
    fn overwrite(&mut self, data: &[T], dst: usize, src: Option<usize>);
}

use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

pub struct TerminalRenderer {
    delay: Duration,
}
impl TerminalRenderer {
    pub fn new(delay_ms: u64) -> Self {
        Self {
            delay: Duration::from_millis(delay_ms),
        }
    }
    fn draw<T: Display>(&self, data: &[T], active: &[usize]) {
        print!("\x1B[2J");
        print!("\x1B[H");

        for (index, value) in data.iter().enumerate() {
            let marker = if active.contains(&index) { ">" } else { " " };

            println!(
                "{} {:>3} {}",
                marker,
                value,
                "█".repeat(value.to_string().parse::<usize>().unwrap_or(1))
            );
        }

        sleep(self.delay);
    }
}
impl<T> Renderer<T> for TerminalRenderer
where
    T: Display,
{
    fn compare(&mut self, data: &[T], i: usize, j: usize) {
        self.draw(data, &[i, j]);
    }
    fn swap(&mut self, data: &[T], i: usize, j: usize) {
        self.draw(data, &[i, j]);
    }
    fn overwrite(&mut self, data: &[T], dst: usize, src: Option<usize>) {
        match src {
            Some(src) => {
                self.draw(data, &[dst, src]);
            }
            None => {
                self.draw(data, &[dst]);
            }
        }
    }
}
