use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn selection_sort<T: Ord + Clone>(v: &mut [T]) -> &mut [T] {
    for i in 0..v.len() - 1 {
        let mut min_idx = i;
        for j in (i + 1)..v.len() {
            if v[j] < v[min_idx] {
                min_idx = j;
            }
        }
        v.swap(i, min_idx);
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut to_sort = vec![87, 10, 2,822, 1, 5, 7, 4, 87 ];
        assert_eq!(selection_sort(&mut to_sort), &[1, 2, 4, 5, 7, 10, 87, 822]);
    }
}
fn main() {
    let mut input: Vec<u64> = (0..100_000)
        .map(|_| rand::rng().random_range(0..1_000_000_000))
        .collect();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    selection_sort(&mut input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Duration: {} ms", end.as_millis() - start.as_millis());
}
