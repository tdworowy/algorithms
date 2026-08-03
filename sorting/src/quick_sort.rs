use crate::observer::{SortObserver, VisualArray};

pub fn quick_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Ord,
    O: SortObserver<T>,
{
    if array.len() <= 1 {
        return;
    }

    let mut arr = VisualArray::new(array, observer);
    let len = arr.len();
    quick_sort_range(&mut arr, 0, len - 1);
}

fn quick_sort_range<T, O>(arr: &mut VisualArray<'_, T, O>, low: usize, high: usize)
where
    T: Ord,
    O: SortObserver<T>,
{
    if low < high {
        let p = partition(arr, low, high);

        if p > 0 {
            quick_sort_range(arr, low, p - 1);
        }
        quick_sort_range(arr, p + 1, high);
    }
}

fn partition<T, O>(arr: &mut VisualArray<'_, T, O>, low: usize, high: usize) -> usize
where
    T: Ord,
    O: SortObserver<T>,
{
    let pivot = high;
    let mut i = low;

    for j in low..high {
        if arr.compare(j, pivot).is_le() {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
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
