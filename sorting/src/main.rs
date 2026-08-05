use crate::observer::{TerminalObserver, TerminalVisualizationObserver};
use crate::renderer::TerminalRenderer;
use rand::{Rng, RngExt};

mod bubble_sort;
mod heap_sort;
mod insertion_and_merge_sort;
mod insertion_sort;
mod merge_sort;
mod observer;
mod quick_sort;
mod renderer;
mod selection_sort;

fn main() {
    let mut rng = rand::rng();
    let renderer = TerminalRenderer::new(200);
    let mut observer = TerminalVisualizationObserver::new(renderer);

    let mut data: Vec<u32> = (0..20).map(|_| rng.random_range(0..70)).collect();
    quick_sort::quick_sort(&mut data, &mut observer);

    let mut data: Vec<u32> = (0..20).map(|_| rng.random_range(0..70)).collect();
    bubble_sort::bubble_sort(&mut data, &mut observer);

    let mut data: Vec<u32> = (0..20).map(|_| rng.random_range(0..70)).collect();
    insertion_sort::insertion_sort(&mut data, &mut observer);

    let mut data: Vec<u32> = (0..20).map(|_| rng.random_range(0..70)).collect();
    heap_sort::heap_sort(&mut data, &mut observer);

    let mut data: Vec<u32> = (0..20).map(|_| rng.random_range(0..70)).collect();
    merge_sort::merge_sort(&mut data, &mut observer);
}
