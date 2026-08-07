use crate::observer::SortObserver;

pub fn merge_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    if array.len() <= 1 {
        return;
    }

    let mut buffer = array.to_vec();
    let len = array.len();

    merge_sort_impl(array, &mut buffer, 0, len - 1, observer);
}

fn merge_sort_impl<T, O>(
    array: &mut [T],
    buffer: &mut [T],
    left: usize,
    right: usize,
    observer: &mut O,
) where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    if left >= right {
        return;
    }

    let mid = left + (right - left) / 2;

    merge_sort_impl(array, buffer, left, mid, observer);
    merge_sort_impl(array, buffer, mid + 1, right, observer);
    merge(array, buffer, left, mid, right, observer);
}

fn merge<T, O>(
    array: &mut [T],
    buffer: &mut [T],
    left: usize,
    mid: usize,
    right: usize,
    observer: &mut O,
) where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    buffer[left..=right].clone_from_slice(&array[left..=right]);
    let mut left_index = left;
    let mut right_index = mid + 1;
    let mut write_index = left;

    while left_index <= mid && right_index <= right {
        observer.compare(buffer, left_index, right_index);

        if buffer[left_index] <= buffer[right_index] {
            array[write_index] = buffer[left_index].clone();
            observer.overwrite(array, write_index, Some(left_index));
            left_index += 1;
        } else {
            array[write_index] = buffer[right_index].clone();
            observer.overwrite(array, write_index, Some(right_index));
            right_index += 1;
        }

        write_index += 1;
    }

    while left_index <= mid {
        array[write_index] = buffer[left_index].clone();
        observer.overwrite(array, write_index, Some(left_index));
        left_index += 1;
        write_index += 1;
    }

    while right_index <= right {
        array[write_index] = buffer[right_index].clone();
        observer.overwrite(array, write_index, Some(right_index));
        right_index += 1;
        write_index += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_merge_basic() {
        let mut arr = vec![1, 3, 5, 2, 4, 6];
        let mut buffer = arr.to_vec();
        merge(&mut *arr, &mut buffer, 0, 2, 5, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let mut buffer = arr.to_vec();
        merge(&mut arr, &mut buffer, 0, 2, 5, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_reverse_halves() {
        let mut arr = vec![4, 5, 6, 1, 2, 3];
        let mut buffer = arr.to_vec();
        merge(&mut arr, &mut buffer, 0, 2, 5, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_with_duplicates() {
        let mut arr = vec![1, 3, 3, 2, 3, 4];
        let mut buffer = arr.to_vec();
        merge(&mut arr, &mut buffer, 0, 2, 5, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 3, 3, 3, 4]);
    }

    #[test]
    fn test_merge_single_element_halves() {
        let mut arr = vec![2, 1];
        let mut buffer = arr.to_vec();
        merge(&mut arr, &mut buffer, 0, 0, 1, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2]);
    }
    #[test]
    fn test_merge_sort1() {
        let mut arr = vec![4, 5, 6, 1, 2, 3];
        merge_sort(&mut arr, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn test_merge_sort2() {
        let mut arr = vec![10, 2, 1, 5, 7, 4, 87, 822, 10, 2];
        merge_sort(&mut arr, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 2, 4, 5, 7, 10, 10, 87, 822]);
    }
}
