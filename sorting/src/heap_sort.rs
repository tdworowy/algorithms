use crate::observer::{SortObserver, VisualArray};

fn left(i: usize) -> usize {
    2 * i + 1
}
fn right(i: usize) -> usize {
    2 * i + 2
}
fn parent(i: usize) -> usize {
    i / 2
}

pub fn heap_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Ord,
    O: SortObserver<T>,
{
    let mut arr = VisualArray::new(array, observer);

    build_max_heap(&mut arr);

    for heap_size in (1..arr.len()).rev() {
        arr.swap(0, heap_size);
        max_heapify(&mut arr, 0, heap_size);
    }
}

fn build_max_heap<T, O>(arr: &mut VisualArray<'_, T, O>)
where
    T: Ord,
    O: SortObserver<T>,
{
    let heap_size = arr.len();

    for i in (0..heap_size / 2).rev() {
        max_heapify(arr, i, heap_size);
    }
}

fn max_heapify<T, O>(arr: &mut VisualArray<'_, T, O>, mut i: usize, heap_size: usize)
where
    T: Ord,
    O: SortObserver<T>,
{
    loop {
        let l = left(i);
        let r = right(i);

        let mut largest = i;

        if l < heap_size && arr.compare(l, largest).is_gt() {
            largest = l;
        }

        if r < heap_size && arr.compare(r, largest).is_gt() {
            largest = r;
        }

        if largest == i {
            break;
        }

        arr.swap(i, largest);
        i = largest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_max_heapify_simple() {
        // Root is smaller than children, so it should be pushed down
        let mut array = vec![1, 5, 3];
        let mut observer = NoOpObserver;
        let mut arr = VisualArray::new(&mut *array, &mut observer);
        let len = arr.len();
        max_heapify(&mut arr, 0, len);

        // After heapify, 5 should be at root
        assert_eq!(arr[0], 5);
        assert!(arr[1] <= arr[0]);
        assert!(arr[2] <= arr[0]);
    }

    #[test]
    fn test_max_heapify_already_heap() {
        let mut array = vec![10, 5, 3];
        let mut binding = array.clone();
        let mut observer1 = NoOpObserver;
        let mut observer2 = NoOpObserver;
        let mut arr1 = VisualArray::new(&mut *array, &mut observer1);
        let arr2 = VisualArray::new(&mut *binding, &mut observer2);
        let len = arr1.len();
        max_heapify(&mut arr1, 0, len);

        // Should remain unchanged
        assert_eq!(arr1.data, arr2.data);
    }

    #[test]
    fn test_max_heapify_larger_tree() {
        let mut array = vec![2, 9, 7, 6, 5, 8];
        let mut observer = NoOpObserver;
        let mut arr = VisualArray::new(&mut *array, &mut observer);
        let len = arr.len();
        max_heapify(&mut arr, 0, len);

        // root must be max
        assert_eq!(arr[0], 9);

        // heap property check for root
        let l = 1;
        let r = 2;

        if l < arr.len() {
            assert!(arr[0] >= arr[l]);
        }
        if r < arr.len() {
            assert!(arr[0] >= arr[r]);
        }
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![128, 3, 6, 2, 1, 8, 9, 2, 3];
        heap_sort(&mut arr, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 6, 8, 9, 128]);
    }
}
