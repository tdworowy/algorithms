use crate::observer::TerminalVisualizationObserver;
use crate::renderer::Animation;
use crate::{bubble_sort, heap_sort, insertion_sort, merge_sort, quick_sort, selection_sort};
use rand::RngExt;
pub(crate) struct SortRunner {
    pub(crate) animation: Animation<u32>,
}

impl SortRunner {
    pub(crate) fn new(choice: usize) -> Self {
        let mut rng = rand::rng();
        let mut data: Vec<u32> = (0..30).map(|_| rng.random_range(1..100)).collect();
        let initial_data = data.clone();

        let mut observer = TerminalVisualizationObserver::new();
        match choice {
            0 => {
                quick_sort::quick_sort(&mut data, &mut observer);
            }
            1 => {
                bubble_sort::bubble_sort(&mut data, &mut observer);
            }
            2 => {
                insertion_sort::insertion_sort(&mut data, &mut observer);
            }
            3 => {
                selection_sort::selection_sort(&mut data, &mut observer);
            }
            4 => {
                heap_sort::heap_sort(&mut data, &mut observer);
            }
            5 => {
                merge_sort::merge_sort(&mut data, &mut observer);
            }
            _ => {}
        }
        let animation = Animation::new(initial_data, observer.events);
        Self { animation }
    }
    pub(crate) fn step(&mut self) {
        self.animation.step();
    }
    pub(crate) fn finished(&self) -> bool {
        self.animation.finished()
    }
}
