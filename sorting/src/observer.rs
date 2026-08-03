use std::fmt::Display;

pub trait SortObserver<T> {
    fn compare(&mut self, _data: &[T], _i: usize, _j: usize) {}
    fn swap(&mut self, _data: &[T], _i: usize, _j: usize) {}
    fn overwrite(&mut self, _data: &[T], _i: usize) {}
}
pub struct NoOpObserver;

impl<T> SortObserver<T> for NoOpObserver {}
pub struct TerminalObserver;

impl<T> SortObserver<T> for TerminalObserver
where
    T: Display,
{
    fn compare(&mut self, data: &[T], i: usize, j: usize) {
        println!("compare [{}] and [{}]", i, j);
        print_array(data);
    }
    fn swap(&mut self, data: &[T], i: usize, j: usize) {
        println!("swap [{}] <-> [{}]", i, j);
        print_array(data);
    }
}

fn print_array<T: Display>(data: &[T]) {
    for x in data {
        print!("{} ", x);
    }
    println!();
    println!();
}
