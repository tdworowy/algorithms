fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);

    let (left, right) = arr.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let last = arr.len() - 1;
    let mut i = 0;

    for j in 0..last {
        if arr[j] <= arr[last] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, last);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut to_sort = vec![10, 2, 822, 1, 5, 7, 4, 87, 88];
        quick_sort(&mut to_sort);
        assert_eq!(to_sort, vec![1, 2, 4, 5, 7, 10, 87, 88, 822])
    }
}
