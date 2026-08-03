

fn partition<T: Ord + Copy>(array: &mut [T], p: usize, r: usize) -> usize {
    let x = array[r];
    let mut i = p - 1;
    for j in p..r {
        if array[j] <= x {
            i += 1;
            array.swap(i, j);
        }
    }
    array.swap(i + 1, r);
    i + 1
}
fn quick_sort<T: Ord + Copy>(array: &mut [T], p: usize, r: usize) {
    if p < r {
        let q = partition(array, p, r);
        quick_sort(array, p, q - 1);
        quick_sort(array, q + 1, r);
    }
}

fn main() {
    println!("Hello, world!");
}
