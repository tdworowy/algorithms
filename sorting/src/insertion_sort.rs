use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn insertion_sort<T: Ord + Clone>(v: &mut [T]) -> &mut [T] {
    for i in 1..v.len() {
        let key = v[i].clone();
        let mut j = i;
        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1].clone();
            j -= 1;
        }
        v[j] = key;
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![10, 2, 1, 5, 7, 4, 87, 822];
        assert_eq!(insertion_sort(&mut v), vec![1, 2, 4, 5, 7, 10, 87, 822]);
    }
}
