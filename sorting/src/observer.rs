use crate::renderer::Renderer;
use std::cmp::Ordering;
use std::fmt::Display;
use std::io::{Write, stdout};

#[derive(Debug, Clone)]
pub enum SortEvent {
    Compare { first: usize, second: usize },

    Swap { first: usize, second: usize },

    Overwrite { index: usize },

    Mark { index: usize, kind: MarkKind },

    Step,
}

#[derive(Debug, Clone)]
pub enum MarkKind {
    Pivot,
    Sorted,
    Active,
    Minimum,
}

pub trait SortObserver<T> {
    fn compare(&mut self, _data: &[T], _i: usize, _j: usize) {}
    fn swap(&mut self, _data: &[T], _i: usize, _j: usize) {}
    fn overwrite(&mut self, _data: &[T], _dst: usize, _src: Option<usize>) {}
}

pub struct NoOpObserver;

impl<T> SortObserver<T> for NoOpObserver {}

pub struct TerminalObserver;

impl<T> SortObserver<T> for TerminalObserver
where
    T: Display,
{
    fn compare(&mut self, data: &[T], i: usize, j: usize) {
        println!("compare [{}] and [{}]", data[i], data[j]);
        print_array(data);
    }

    fn swap(&mut self, data: &[T], i: usize, j: usize) {
        println!("swap [{}] <-> [{}]", data[i], data[j]);
        print_array(data);
    }

    fn overwrite(&mut self, data: &[T], dst: usize, src: Option<usize>) {
        match src {
            Some(s) => println!("shift {} -> {}", s, dst),
            None => println!("write at {}", dst),
        }
        print_array(data);
    }
}

pub struct TerminalVisualizationObserver<R> {
    renderer: R,
}

impl<R> TerminalVisualizationObserver<R> {
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }
}
impl<T, R> SortObserver<T> for TerminalVisualizationObserver<R>
where
    R: Renderer<T>,
{
    fn compare(&mut self, data: &[T], i: usize, j: usize) {
        self.renderer.compare(data, i, j);
    }
    fn swap(&mut self, data: &[T], i: usize, j: usize) {
        self.renderer.swap(data, i, j);
    }
    fn overwrite(&mut self, data: &[T], dst: usize, src: Option<usize>) {
        self.renderer.overwrite(data, dst, src);
    }
}

fn print_array<T: Display>(data: &[T]) {
    for x in data {
        print!("{} ", x);
    }
    println!();
}

pub struct VisualArray<'a, T, O> {
    pub(crate) data: &'a mut [T],
    pub(crate) observer: &'a mut O,
}

impl<'a, T, O> VisualArray<'a, T, O>
where
    O: SortObserver<T>,
{
    pub fn new(data: &'a mut [T], observer: &'a mut O) -> Self {
        Self { data, observer }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn compare(&mut self, i: usize, j: usize) -> Ordering
    where
        T: Ord,
    {
        self.observer.compare(self.data, i, j);
        self.data[i].cmp(&self.data[j])
    }

    pub fn compare_value(&mut self, i: usize, value: &T) -> Ordering
    where
        T: Ord,
    {
        self.data[i].cmp(value)
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
        self.observer.swap(self.data, i, j);
    }

    pub fn overwrite(&mut self, dst: usize, value: T) {
        self.data[dst] = value;
        self.observer.overwrite(self.data, dst, None);
    }

    pub fn shift(&mut self, dst: usize, src: usize)
    where
        T: Clone,
    {
        self.data[dst] = self.data[src].clone();
        self.observer.overwrite(self.data, dst, Some(src));
    }
}

impl<'a, T, O> std::ops::Index<usize> for VisualArray<'a, T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a, T, O> std::ops::IndexMut<usize> for VisualArray<'a, T, O> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
