use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SortEvent<T> {
    Compare {
        first: usize,
        second: usize,
    },
    Swap {
        first: usize,
        second: usize,
    },
    Overwrite {
        index: usize,
        value: T,
    },
    Counting {
        counts: Vec<usize>,
        output: Vec<T>,
        state: CountingState<T>,
        exp: Option<u32>,
    },
}

#[derive(Debug, Clone)]
pub enum CountingState<T> {
    Counting,
    Summing,
    Placing { current_val: T },
}

pub trait SortObserver<T> {
    fn compare(&mut self, _data: &[T], _i: usize, _j: usize) {}
    fn swap(&mut self, _data: &[T], _i: usize, _j: usize) {}
    fn overwrite(&mut self, _data: &[T], _dst: usize, _src: Option<usize>) {}
    fn counting(
        &mut self,
        _counts: &[usize],
        _output: &[T],
        _state: CountingState<T>,
        _exp: Option<u32>,
    ) {
    }
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

pub struct TerminalVisualizationObserver<T> {
    pub(crate) events: Vec<SortEvent<T>>,
}
impl<T> SortObserver<T> for TerminalVisualizationObserver<T>
where
    T: Clone + Display,
{
    fn compare(&mut self, _data: &[T], i: usize, j: usize) {
        self.events.push(SortEvent::Compare {
            first: i,
            second: j,
        });
    }
    fn swap(&mut self, _data: &[T], i: usize, j: usize) {
        self.events.push(SortEvent::Swap {
            first: i,
            second: j,
        });
    }
    fn overwrite(&mut self, data: &[T], dst: usize, _src: Option<usize>) {
        self.events.push(SortEvent::Overwrite {
            index: dst,
            value: data[dst].clone(),
        });
    }

    fn counting(
        &mut self,
        counts: &[usize],
        output: &[T],
        state: CountingState<T>,
        exp: Option<u32>,
    ) {
        self.events.push(SortEvent::Counting {
            counts: counts.to_vec(),
            output: output.to_vec(),
            state,
            exp,
        });
    }
}
impl<T> TerminalVisualizationObserver<T> {
    pub fn new() -> Self {
        Self { events: Vec::new() }
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
