use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn bubble_sort<T: Ord>(v: &mut [T]) -> &mut [T] {
    let len = v.len();
    for i in 0..len {
        let mut swapped = false;

        for j in ((i + 1)..len).rev() {
            if v[j] < v[j - 1] {
                v.swap(j, j - 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![3, 5, 1, 6, 4];
        assert_eq!(bubble_sort(&mut v), [1, 3, 4, 5, 6]);
    }
}
