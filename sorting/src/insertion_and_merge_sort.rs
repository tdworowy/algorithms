use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn insertion_sort<T: Ord + Copy>(v: &mut [T]) -> &mut [T] {
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
fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}

fn merge_sort<T: Ord + Copy>(array: &mut [T], k: usize) -> Vec<T> {
    let n = array.len();
    if n < 2 {
        return array.to_vec();
    }
    if n < k {
        insertion_sort(array).to_vec()
    } else {
        let mid = n / 2;
        let mut left = array[..mid].to_vec();
        let mut right = array[mid..].to_vec();

        let left_sorted = merge_sort(&mut left, k);
        let right_sorted = merge_sort(&mut right, k);
        let new_array = merge(left_sorted.as_ref(), right_sorted.as_ref());

        new_array
    }
}
