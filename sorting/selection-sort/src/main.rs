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

fn main() {
    let mut to_sort = vec![10, 2, 1, 5, 7, 4, 87, 822];
    println!("{:?}", selection_sort(&mut to_sort));
}
