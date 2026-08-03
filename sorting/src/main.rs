use crate::observer::TerminalObserver;
use crate::quick_sort::quick_sort;

mod bubble_sort;
mod heap_sort;
mod insertion_and_merge_sort;
mod insertion_sort;
mod merge_sort;
mod observer;
mod quick_sort;
mod selection_sort;

fn main() {
    let mut data = vec![9, 4, 7, 3, 10, 2, 6];

    println!("Before:");
    println!("{:?}", data);

    let mut observer = TerminalObserver;
    quick_sort(&mut data, &mut observer);

    println!("After:");
    println!("{:?}", data);
}
