use crate::observer::{SortObserver, VisualArray};
use rand::RngExt;

pub fn bubble_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Ord,
    O: SortObserver<T>,
{
    let mut arr = VisualArray::new(array, observer);
    let len = arr.len();

    for i in 0..len {
        let mut swapped = false;

        for j in ((i + 1)..len).rev() {
            if arr.compare(j, j - 1).is_lt() {
                arr.swap(j, j - 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_bubble_sort() {
        let mut array = vec![3, 5, 1, 6, 4];
        bubble_sort(&mut array, &mut NoOpObserver);
        assert_eq!(array, [1, 3, 4, 5, 6]);
    }
}
