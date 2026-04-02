fn insertion_sort<T: Ord + Clone>(v: &mut [T]) -> &mut [T] {
    for i in 1..v.len() {
        let key = v[i].clone();
        let mut j = i;
        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1].clone();
            j -= 1;
        }
        v[j] = key;
    }
    v
}

fn main() {
    let mut to_sort = vec![10, 2, 1, 5, 7, 4, 87, 822];
    println!("{:?}", insertion_sort(&mut to_sort));
}
