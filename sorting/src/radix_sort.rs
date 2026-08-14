use crate::observer::{CountingState, SortObserver};
pub fn radix_sort<O>(arr: &mut [u32], observer: &mut O)
where
    O: SortObserver<u32>,
{
    if arr.len() <= 1 {
        return;
    }
    let max = *arr.iter().max().unwrap();
    let mut exp = 1;
    while max / exp > 0 {
        counting_sort_by_digit(arr, exp, observer);
        exp *= 10;
    }
}

fn counting_sort_by_digit<O>(arr: &mut [u32], exp: u32, observer: &mut O)
where
    O: SortObserver<u32>,
{
    let n = arr.len();
    let mut output = vec![0u32; n];
    let mut count = [0usize; 10]; // digits 0-9

    for &x in arr.iter() {
        let digit = ((x / exp) % 10) as usize;
        count[digit] += 1;
        observer.counting(&count, &output, CountingState::Counting, Some(exp));
    }
    for i in 1..10 {
        count[i] += count[i - 1];
        observer.counting(&count, &output, CountingState::Summing, Some(exp));
    }
    for &x in arr.iter().rev() {
        let digit = ((x / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = x;
        observer.counting(
            &count,
            &output,
            CountingState::Placing { current_val: x },
            Some(exp),
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
        let mut array = vec![231, 3, 5, 1, 6, 4, 6, 5, 99];
        radix_sort(&mut array, &mut NoOpObserver);
        assert_eq!(array, [1, 3, 4, 5, 5, 6, 6, 99, 231]);
    }
}
