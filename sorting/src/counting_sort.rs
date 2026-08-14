use crate::observer::{CountingState, SortObserver};

pub fn counting_sort<O, T>(arr: &[T], observer: &mut O) -> Vec<T>
where
    T: Copy + TryInto<usize> + TryFrom<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
    O: SortObserver<T>,
{
    if arr.is_empty() {
        return Vec::new();
    }
    let max = arr.iter().map(|&x| x.try_into().unwrap()).max().unwrap();
    let mut count = vec![0usize; max + 1];
    let mut output = vec![T::try_from(0).unwrap(); arr.len()];

    for &x in arr {
        count[x.try_into().unwrap()] += 1;
        observer.counting(&count, &output, CountingState::Counting, None);
    }
    for i in 1..count.len() {
        count[i] += count[i - 1];
        observer.counting(&count, &output, CountingState::Summing, None);
    }
    for &x in arr.iter().rev() {
        let val: usize = x.try_into().unwrap();
        count[val] -= 1;
        output[count[val]] = x;
        observer.counting(
            &count,
            &output,
            CountingState::Placing { current_val: x },
            None,
        );
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::counting_sort::counting_sort;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_counting_sort() {
        let mut array = vec![3, 5, 1, 6, 4, 6, 5];
        let result = counting_sort(&mut array, &mut NoOpObserver);
        assert_eq!(result, [1, 3, 4, 5, 5, 6, 6]);
    }
}
