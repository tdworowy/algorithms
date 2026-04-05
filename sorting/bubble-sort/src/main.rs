use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn bubble_sort<T: Ord>(v: &mut [T]) -> &mut [T] {
    let len = v.len();
    for i in 0..len {
        let mut swapped = false;

        for j in ((i + 1)..len).rev() {
            if v[j] < v[j - 1] {
                v.swap(j, j - 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![3, 5, 1, 6, 4];
        assert_eq!(bubble_sort(&mut v), [1, 3, 4, 5, 6]);
    }
}
fn main() {
    let mut input: Vec<u64> = (0..30_000)
        .map(|_| rand::rng().random_range(0..1_000_000_000))
        .collect();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    bubble_sort(&mut input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Duration: {} ms", end.as_millis() - start.as_millis());
}
