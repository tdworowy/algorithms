fn left(i: usize) -> usize {
    2 * i + 1
}
fn right(i: usize) -> usize {
    2 * i + 2
}
fn parent(i: usize) -> usize {
    i / 2
}

fn max_heapify<T: Ord + Copy>(array: &mut [T], i: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = if l < array.len() && array[l] > array[i] {
        l
    } else {
        i
    };
    if r < array.len() && array[r] > array[largest] {
        largest = r;
    }
    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest);
    }
}

fn max_heapify_loop<T: Ord + Copy>(array: &mut [T], mut i: usize) {
    while true {
        let l = left(i);
        let r = right(i);
        let mut largest = if l < array.len() && array[l] > array[i] {
            l
        } else {
            i
        };
        if r < array.len() && array[r] > array[largest] {
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
    for i in (0..array.len() / 2).rev() {
        max_heapify(array, i);
    }
}
fn build_max_heap_loop<T: Ord + Copy>(array: &mut [T]) {
    for i in (0..array.len() / 2).rev() {
        max_heapify_loop(array, i);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heapify_simple() {
        // Root is smaller than children, so it should be pushed down
        let mut arr = vec![1, 5, 3];

        max_heapify(&mut arr, 0);

        // After heapify, 5 should be at root
        assert_eq!(arr[0], 5);
        assert!(arr[1] <= arr[0]);
        assert!(arr[2] <= arr[0]);
    }
    #[test]
    fn test_max_heapify_loop_simple() {
        // Root is smaller than children, so it should be pushed down
        let mut arr = vec![1, 5, 3];

        max_heapify_loop(&mut arr, 0);

        // After heapify, 5 should be at root
        assert_eq!(arr[0], 5);
        assert!(arr[1] <= arr[0]);
        assert!(arr[2] <= arr[0]);
    }

    #[test]
    fn test_max_heapify_already_heap() {
        let mut arr = vec![10, 5, 3];

        let before = arr.clone();
        max_heapify(&mut arr, 0);

        // Should remain unchanged
        assert_eq!(arr, before);
    }

    #[test]
    fn test_max_heapify_loop_already_heap() {
        let mut arr = vec![10, 5, 3];

        let before = arr.clone();
        max_heapify_loop(&mut arr, 0);

        // Should remain unchanged
        assert_eq!(arr, before);
    }

    #[test]
    fn test_max_heapify_larger_tree() {
        let mut arr = vec![2, 9, 7, 6, 5, 8];

        max_heapify(&mut arr, 0);

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

        max_heapify_loop(&mut arr, 0);

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
}
fn main() {
    let mut array1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    build_max_heap(&mut array1);
    println!("result {:?}", array1);

    let mut array2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    build_max_heap_loop(&mut array2);
    println!("result {:?}", array2);
}
