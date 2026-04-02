fn merge<T: Ord + Copy>(array: &mut [T], p: usize, q: usize, r: usize) -> &mut [T] {
    let left_len = q - p + 1;
    let right_len = r - q;
    let left = array[p..=q].to_vec();
    let right = array[q + 1..=r].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left_len {
        array[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right_len {
        array[k] = right[j];
        j += 1;
        k += 1;
    }
    array
}

fn merge_sort<T: Ord + Copy>(array: &mut [T], p: usize, r: usize) -> &mut [T] {
    if p >= r {
        return array;
    };
    let q = p + (r - p) / 2;
    merge_sort(array, p, q);
    merge_sort(array, q + 1, r);
    let new_array = merge(array, p, q, r);

    new_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic() {
        let mut arr = vec![1, 3, 5, 2, 4, 6];
        // left: [1,3,5], right: [2,4,6]
        merge(&mut arr, 0, 2, 5);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        merge(&mut arr, 0, 2, 5);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_reverse_halves() {
        let mut arr = vec![4, 5, 6, 1, 2, 3];
        merge(&mut arr, 0, 2, 5);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_with_duplicates() {
        let mut arr = vec![1, 3, 3, 2, 3, 4];
        merge(&mut arr, 0, 2, 5);
        assert_eq!(arr, vec![1, 2, 3, 3, 3, 4]);
    }

    #[test]
    fn test_merge_single_element_halves() {
        let mut arr = vec![2, 1];
        merge(&mut arr, 0, 0, 1);
        assert_eq!(arr, vec![1, 2]);
    }
}

fn main() {
    let mut arr = vec![1, 3, 5, 2, 4, 6];
    let arr_len = arr.len();
    let result = merge_sort(&mut arr, 0, arr_len - 1);
    println!("{:?}", result);
}
