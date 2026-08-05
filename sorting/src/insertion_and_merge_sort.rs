use crate::observer::SortObserver;
use crate::{insertion_sort, merge_sort};

fn insertion_and_merge_sort<T, O>(array: &mut [T], k: usize, observer: &mut O) -> Vec<T>
where
    T: Ord + Clone + Copy,
    O: SortObserver<T>,
{
    let n = array.len();
    if n < 2 {
        return array.to_vec();
    }
    if n < k {
        insertion_sort::insertion_sort(array, observer);
        array.to_vec()
    } else {
        merge_sort::merge_sort(array, observer);
        array.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_insertion_and_merge_sort1() {
        let mut array = vec![10, 2, 1, 5, 7, 4, 87, 822];
        insertion_and_merge_sort(&mut array, 50, &mut NoOpObserver);
        assert_eq!(array, vec![1, 2, 4, 5, 7, 10, 87, 822]);
    }
    #[test]
    fn test_insertion_and_merge_sort2() {
        let mut array = vec![10, 2, 1, 5, 7, 4, 87, 822];
        insertion_and_merge_sort(&mut array, 5, &mut NoOpObserver);
        assert_eq!(array, vec![1, 2, 4, 5, 7, 10, 87, 822]);
    }
}
