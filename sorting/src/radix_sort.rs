use crate::observer::{CountingState, SortObserver};

pub fn radix_sort<T, O>(arr: &mut [T], observer: &mut O)
where
    T: Copy + Ord + Into<u64> + TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
    O: SortObserver<T>,
{
    if arr.len() <= 1 {
        return;
    }
    let max = *arr.iter().max().unwrap();
    let max_u64: u64 = max.into();
    let mut exp: u64 = 1;
    while max_u64 / exp > 0 {
        counting_sort_by_digit(arr, exp, observer);
        if let Some(next_exp) = exp.checked_mul(10) {
            exp = next_exp;
        } else {
            break;
        }
    }
}

fn counting_sort_by_digit<T, O>(arr: &mut [T], exp: u64, observer: &mut O)
where
    T: Copy + Into<u64> + TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
    O: SortObserver<T>,
{
    let n = arr.len();
    let mut output = vec![T::try_from(0).unwrap(); n];
    let mut count = [0usize; 10]; // digits 0-9

    for &x in arr.iter() {
        let val: u64 = x.into();
        let digit = ((val / exp) % 10) as usize;
        count[digit] += 1;
        observer.counting(&count, &output, CountingState::Counting, Some(exp as u32));
    }
    for i in 1..10 {
        count[i] += count[i - 1];
        observer.counting(&count, &output, CountingState::Summing, Some(exp as u32));
    }
    for &x in arr.iter().rev() {
        let val: u64 = x.into();
        let digit = ((val / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = x;
        observer.counting(
            &count,
            &output,
            CountingState::Placing { current_val: x },
            Some(exp as u32),
        );
    }
    arr.copy_from_slice(&output);
}
#[cfg(test)]
mod tests {
    use crate::observer::NoOpObserver;
    use crate::radix_sort::radix_sort;

    #[test]
    fn test_radix_sort() {
        let mut array: Vec<u32> = vec![231, 3, 5, 1, 6, 4, 6, 5, 99];
        radix_sort(&mut array, &mut NoOpObserver);
        assert_eq!(array, [1, 3, 4, 5, 5, 6, 6, 99, 231]);
    }
}
