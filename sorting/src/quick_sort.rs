use crate::observer::SortObserver;

pub fn quick_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Ord,
    O: SortObserver<T>,
{
    if array.len() <= 1 {
        return;
    }
    let len = array.len();
    quick_sort_range(array, 0, len - 1, observer);
}
fn quick_sort_range<T, O>(array: &mut [T], low: usize, high: usize, observer: &mut O)
where
    T: Ord,
    O: SortObserver<T>,
{
    if low < high {
        let p = partition(array, low, high, observer);

        if p > 0 {
            quick_sort_range(array, low, p - 1, observer);
        }

        quick_sort_range(array, p + 1, high, observer);
    }
}

fn partition<T, O>(array: &mut [T], low: usize, high: usize, observer: &mut O) -> usize
where
    T: Ord,
    O: SortObserver<T>,
{
    let pivot = high;
    let mut i = low;

    for j in low..high {
        observer.compare(array, j, pivot);
        if array[j] <= array[pivot] {
            array.swap(i, j);
            observer.swap(array, i, j);
            i += 1;
        }
    }
    array.swap(i, high);
    observer.swap(array, i, high);
    i
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_quick_sort() {
        let mut to_sort = vec![10, 2, 822, 1, 5, 7, 4, 87, 88];
        quick_sort(&mut to_sort, &mut NoOpObserver);
        assert_eq!(to_sort, vec![1, 2, 4, 5, 7, 10, 87, 88, 822])
    }
}
