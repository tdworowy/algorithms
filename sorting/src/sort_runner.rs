use crate::observer;
use crate::renderer::Animation;
use crate::{
    bubble_sort, counting_sort, heap_sort, insertion_sort, merge_sort, quick_sort, radix_sort,
    selection_sort,
};
use rand::RngExt;
pub(crate) struct SortRunner {
    pub(crate) animation: Animation<u32>,
}

impl SortRunner {
    pub(crate) fn new(choice: usize) -> Self {
        let mut rng = rand::rng();
        let mut data: Vec<u32> = (0..60).map(|_| rng.random_range(1..100)).collect();
        let initial_data = data.clone();

        let mut observer = observer::TerminalVisualizationObserver::new();
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
            6 => {
                let data_usize: Vec<usize> = data.iter().map(|&x| x as usize).collect();
                let mut counting_observer = observer::TerminalVisualizationObserver::<usize>::new();
                counting_sort::counting_sort(&data_usize, &mut counting_observer);

                let events = counting_observer
                    .events
                    .into_iter()
                    .map(|e| match e {
                        observer::SortEvent::Counting {
                            counts,
                            output,
                            state,
                            exp,
                        } => observer::SortEvent::Counting {
                            counts,
                            output: output.into_iter().map(|x| x as u32).collect(),
                            state: match state {
                                observer::CountingState::Counting => {
                                    observer::CountingState::Counting
                                }
                                observer::CountingState::Summing => {
                                    observer::CountingState::Summing
                                }
                                observer::CountingState::Placing { current_val } => {
                                    observer::CountingState::Placing {
                                        current_val: current_val as u32,
                                    }
                                }
                            },
                            exp,
                        },
                        _ => unreachable!(),
                    })
                    .collect();

                let animation = Animation::new(data, events);
                return Self { animation };
            }
            7 => {
                radix_sort::radix_sort(&mut data, &mut observer);
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
