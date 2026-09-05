use rand::RngExt;

fn partition<T: Ord + Copy>(array: &mut [T], p: usize, r: usize) -> usize {
    let pivot = array[r];
    let mut i = p;
    for j in p..r {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, r);
    i
}

fn randomized_partition<T: Ord + Copy>(array: &mut [T], p: usize, r: usize) -> usize {
    let mut rng = rand::rng();
    let i = rng.random_range(p..=r);
    array.swap(i, r);
    partition(array, p, r)
}

fn randomized_selection<T: Ord + Copy + std::fmt::Debug>(
    array: &mut [T],
    p: usize,
    r: usize,
    i: usize,
) -> T {
    println!(
        "selection: [{p}..={r}] = {:?}, looking for {i}",
        &array[p..=r]
    );
    if p == r {
        return array[p];
    }
    let q = randomized_partition(array, p, r);
    let k = q - p + 1;

    if i == k {
        array[q]
    } else if i < k {
        randomized_selection(array, p, q - 1, i)
    } else {
        randomized_selection(array, q + 1, r, i - k)
    }
}

fn main() {
    let arr = vec![7, 23, 3, 73, 1, 123, 243, 124, 2];
    println!(
        "Third smallest element: {:?}",
        randomized_selection(&mut arr.clone(), 0, arr.len() - 1, 3)
    );
    println!(
        "Smallest element: {:?}",
        randomized_selection(&mut arr.clone(), 0, arr.len() - 1, 1)
    );
    println!(
        "Biggest element: {:?}",
        randomized_selection(&mut arr.clone(), 0, arr.len() - 1, arr.len())
    );
    println!(
        "Median: {:?}",
        randomized_selection(&mut arr.clone(), 0, arr.len() - 1, (arr.len() / 2) + 1)
    );
}
