fn left(i: usize) -> usize {
    2 * i + 1
}
fn right(i: usize) -> usize {
    2 * i + 2
}
fn parent(i: usize) -> usize {
    i / 2
}

fn max_heapify<T: Ord + Copy>(array: &mut [T], i: usize, heap_size: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = if l < heap_size && array[l] > array[i] {
        l
    } else {
        i
    };
    if r < heap_size && array[r] > array[largest] {
        largest = r;
    }
    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest, heap_size);
    }
}

fn max_heapify_loop<T: Ord + Copy>(array: &mut [T], mut i: usize, heap_size: usize) {
    loop {
        let l = left(i);
        let r = right(i);
        let mut largest = if l < heap_size && array[l] > array[i] {
            l
        } else {
            i
        };
        if r < heap_size && array[r] > array[largest] {
            largest = r;
        }
        if largest != i {
            array.swap(i, largest);
            i = largest;
        } else {
            break;
        }
    }
}

fn build_max_heap<T: Ord + Copy>(array: &mut [T]) {
    let heap_size = array.len();
    for i in (0..heap_size / 2).rev() {
        max_heapify(array, i, heap_size);
    }
}
fn build_max_heap_loop<T: Ord + Copy>(array: &mut [T]) {
    let heap_size = array.len();
    for i in (0..heap_size / 2).rev() {
        max_heapify_loop(array, i, heap_size);
    }
}

fn heap_sort<T: Ord + Copy>(array: &mut [T]) {
    build_max_heap(array);
    for heap_size in (1..array.len()).rev() {
        array.swap(0, heap_size);
        max_heapify(array, 0, heap_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heapify_simple() {
        // Root is smaller than children, so it should be pushed down
        let mut arr = vec![1, 5, 3];
        let len = arr.len();
        max_heapify(&mut arr, 0, len);

        // After heapify, 5 should be at root
        assert_eq!(arr[0], 5);
        assert!(arr[1] <= arr[0]);
        assert!(arr[2] <= arr[0]);
    }
    #[test]
    fn test_max_heapify_loop_simple() {
        // Root is smaller than children, so it should be pushed down
        let mut arr = vec![1, 5, 3];
        let len = arr.len();
        max_heapify_loop(&mut arr, 0, len);

        // After heapify, 5 should be at root
        assert_eq!(arr[0], 5);
        assert!(arr[1] <= arr[0]);
        assert!(arr[2] <= arr[0]);
    }

    #[test]
    fn test_max_heapify_already_heap() {
        let mut arr = vec![10, 5, 3];
        let len = arr.len();
        let before = arr.clone();
        max_heapify(&mut arr, 0, len);

        // Should remain unchanged
        assert_eq!(arr, before);
    }

    #[test]
    fn test_max_heapify_loop_already_heap() {
        let mut arr = vec![10, 5, 3];
        let len = arr.len();
        let before = arr.clone();
        max_heapify_loop(&mut arr, 0, len);

        // Should remain unchanged
        assert_eq!(arr, before);
    }

    #[test]
    fn test_max_heapify_larger_tree() {
        let mut arr = vec![2, 9, 7, 6, 5, 8];
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
    fn test_max_heapify_loop_larger_tree() {
        let mut arr = vec![2, 9, 7, 6, 5, 8];
        let len = arr.len();
        max_heapify_loop(&mut arr, 0, len);

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
        let mut arr = vec![128, 3, 6, 2, 1, 8, 9];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 6, 8, 9, 128]);
    }
}
fn main() {
    let mut array1 = [128, 3, 6, 2, 1, 8, 9];
    heap_sort(&mut array1);
    println!("{:?}", array1);
}
