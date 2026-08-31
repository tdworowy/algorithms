use crate::insertion_sort_bucket;
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
    observer.bucket_start(bucket_count);

    let min_value = array
        .iter()
        .map(|value| value.clone().try_into().unwrap())
        .min()
        .unwrap();

    let max_value = array
        .iter()
        .map(|value| value.clone().try_into().unwrap())
        .max()
        .unwrap();

    let range = max_value - min_value + 1;
    let mut buckets: Vec<Vec<T>> = vec![Vec::new(); bucket_count];

    // Distribute elements into buckets.
    for (index, value) in array.iter().enumerate() {
        let numeric_value: usize = value.clone().try_into().unwrap();

        let bucket = ((numeric_value - min_value) * bucket_count) / range;

        // Prevent bucket_count from becoming a valid-looking
        // but actually out-of-bounds index.
        let bucket = bucket.min(bucket_count - 1);
        buckets[bucket].push(value.clone());
        observer.bucket(index, bucket);
    }
    // Sort each bucket.
    for (bucket_index, bucket) in buckets.iter_mut().enumerate() {
        insertion_sort_bucket::insertion_sort_bucket(bucket, bucket_index, observer);
    }
    // Concatenate buckets back into the original array.
    let mut array_index = 0;
    for bucket in buckets.iter() {
        for value in bucket {
            array[array_index] = value.clone();
            observer.overwrite(array, array_index, None);

            array_index += 1;
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
