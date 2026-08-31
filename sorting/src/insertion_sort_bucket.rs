use crate::observer::SortObserver;

pub fn insertion_sort_bucket<T, O>(bucket: &mut [T], bucket_index: usize, observer: &mut O)
where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    for current_index in 1..bucket.len() {
        let key = bucket[current_index].clone();
        let mut insertion_index = current_index;

        while insertion_index > 0 {
            let previous_index = insertion_index - 1;

            observer.bucket_compare(bucket_index, previous_index, insertion_index);

            if bucket[previous_index] <= key {
                break;
            }
            let value = bucket[previous_index].clone();
            bucket[insertion_index] = value.clone();
            observer.bucket_overwrite(bucket_index, insertion_index, value);

            insertion_index -= 1;
        }
        bucket[insertion_index] = key.clone();
        observer.bucket_overwrite(bucket_index, insertion_index, key);
    }
}
