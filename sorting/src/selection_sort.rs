use crate::observer::SortObserver;

pub(crate) fn selection_sort<T, O>(v: &mut [T], observer: &mut O)
where
    T: Ord + Clone,
    O: SortObserver<T>,
{
    let len = v.len();
    if len <= 1 {
        return;
    }
    for i in 0..len - 1 {
        let mut min_idx = i;
        for j in (i + 1)..len {
            observer.compare(v, j, min_idx);
            if v[j] < v[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            v.swap(i, min_idx);
            observer.swap(v, i, min_idx);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::NoOpObserver;

    #[test]
    fn test_selection_sort() {
        let mut to_sort = vec![87, 10, 2, 822, 1, 5, 7, 4, 87];
        selection_sort(&mut to_sort, &mut NoOpObserver);
        assert_eq!(to_sort, &[1, 2, 4, 5, 7, 10, 87, 87, 822]);
    }
}
