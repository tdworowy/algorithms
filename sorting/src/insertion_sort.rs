use crate::observer::{SortObserver, VisualArray};

pub fn insertion_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    let mut arr = VisualArray::new(array, observer);

    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;

        while j > 0 {
            arr.observer.compare(arr.data, j - 1, i);
            if arr.compare_value(j - 1, &key).is_le() {
                break;
            }
            arr.shift(j, j - 1);
            j -= 1;
        }

        arr.overwrite(j, key);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_insertion_sort1() {
        let mut array = vec![10, 2, 1, 5, 7, 4, 87, 822];
        insertion_sort(&mut array, &mut NoOpObserver);
        assert_eq!(array, vec![1, 2, 4, 5, 7, 10, 87, 822]);
    }
}
