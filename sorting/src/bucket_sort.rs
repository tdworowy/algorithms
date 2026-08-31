use crate::insertion_sort::insertion_sort;
use crate::observer::SortObserver;

pub fn bucket_sort<T, O>(array: &mut [T], observer: &mut O)
where
    T: Copy + Ord + TryInto<usize> + TryFrom<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
    O: SortObserver<T>,
{
    if array.len() <= 1 {
        return;
    }
    let bucket_count = array.len();
    let min_value = *array.iter().min().unwrap();
    let max_value = *array.iter().max().unwrap();
    let min: usize = min_value.try_into().unwrap();
    let max: usize = max_value.try_into().unwrap();
    let range = max - min + 1;
    let mut buckets: Vec<Vec<T>> = vec![Vec::new(); bucket_count];

    // Distribute
    for &value in array.iter() {
        let numeric_value: usize = value.try_into().unwrap();
        let bucket_index = ((numeric_value - min) * bucket_count) / range;
        let bucket_index = bucket_index.min(bucket_count - 1);
        buckets[bucket_index].push(value);
    }
    // Sort buckets
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket, observer);
    }
    // Collect
    let mut output_index = 0;
    for bucket in buckets {
        for value in bucket {
            array[output_index] = value;
            observer.overwrite(array, output_index, None);
            output_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_bucket_sort1() {
        let mut array = vec![3, 5, 1, 6, 4, 6, 5, 5, 5, 5];
        bucket_sort(&mut array, &mut NoOpObserver);
        assert_eq!(array, [1, 3, 4, 5, 5, 5, 5, 5, 6, 6]);
    }
    #[test]
    fn test_bucket_sort2() {
        let mut array = vec![3, 10, 20, 50];
        bucket_sort(&mut array, &mut NoOpObserver);
        assert_eq!(array, [3, 10, 20, 50]);
    }
}
