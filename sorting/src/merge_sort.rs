use crate::observer::SortObserver;
use rand::RngExt;

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

fn merge_sort_impl<T, O>(array: &mut [T], buffer: &mut [T], p: usize, r: usize, observer: &mut O)
where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    if p >= r {
        return;
    }

    let q = p + (r - p) / 2;

    merge_sort_impl(array, buffer, p, q, observer);
    merge_sort_impl(array, buffer, q + 1, r, observer);
    merge(array, buffer, p, q, r, observer);
}

fn merge<T, O>(array: &mut [T], buffer: &mut [T], p: usize, q: usize, r: usize, observer: &mut O)
where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    buffer[p..=r].clone_from_slice(&array[p..=r]);
    let mut i = p;
    let mut j = q + 1;
    let mut k = p;

    while i <= q && j <= r {
        observer.compare(buffer, i, j);

        if buffer[i] <= buffer[j] {
            array[k] = buffer[i].clone();
            observer.overwrite(array, k, Some(i));
            i += 1;
        } else {
            array[k] = buffer[j].clone();
            observer.overwrite(array, k, Some(j));
            j += 1;
        }

        k += 1;
    }

    while i <= q {
        array[k] = buffer[i].clone();
        observer.overwrite(array, k, Some(i));
        i += 1;
        k += 1;
    }

    while j <= r {
        array[k] = buffer[j].clone();
        observer.overwrite(array, k, Some(j));
        j += 1;
        k += 1;
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
        let mut arr = vec![10, 2, 1, 5, 7, 4, 87, 822];
        merge_sort(&mut arr, &mut NoOpObserver);
        assert_eq!(arr, vec![1, 2, 4, 5, 7, 10, 87, 822]);
    }
}
