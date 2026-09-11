fn partition_around<T>(array: &mut [T], low: usize, high: usize, pivot: usize) -> usize
where
    T: Ord + Clone,
{
    array.swap(pivot, high);
    let pivot_value = array[high].clone();
    let mut i = low;
    for j in low..high {
        if array[j] <= pivot_value {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, high);
    i
}

fn sort_group<T: Ord + Copy>(array: &mut [T], j: usize, g: usize) {
    let mut group = [
        array[j],
        array[j + g],
        array[j + 2 * g],
        array[j + 3 * g],
        array[j + 4 * g],
    ];

    group.sort_unstable();

    array[j] = group[0];
    array[j + g] = group[1];
    array[j + 2 * g] = group[2];
    array[j + 3 * g] = group[3];
    array[j + 4 * g] = group[4];
}

fn selection<T>(array: &mut [T], mut p: usize, r: usize, mut i: usize) -> T
where
    T: Ord + Copy,
{
    while (r - p + 1) % 5 != 0 {
        for j in (p + 1)..=r {
            if array[p] > array[j] {
                array.swap(p, j);
            }
        }
        if i == 1 {
            return array[p];
        }
        p += 1;
        i -= 1;
    }
    let g = (r - p + 1) / 5;
    for j in p..(p + g) {
        sort_group(array, j, g);
    }
    let x = selection(array, p + 2 * g, p + 3 * g - 1, (g + 1) / 2);
    let pivot_index = (p..=r).find(|&index| array[index] == x).unwrap();
    let q = partition_around(array, p, r, pivot_index);
    let k = q - p + 1;
    if i == k {
        array[q]
    } else if i < k {
        return selection(array, p, q - 1, i);
    } else {
        return selection(array, q + 1, r, i - k);
    }
}

fn main() {
    let arr = vec![7, 23, 3, 73, 1, 123, 243, 124, 2];
    println!(
        "Third smallest element: {:?}",
        selection(&mut arr.clone(), 0, arr.len() - 1, 3)
    );
    println!(
        "Smallest element: {:?}",
        selection(&mut arr.clone(), 0, arr.len() - 1, 1)
    );
    println!(
        "Biggest element: {:?}",
        selection(&mut arr.clone(), 0, arr.len() - 1, arr.len())
    );
    println!(
        "Median: {:?}",
        selection(&mut arr.clone(), 0, arr.len() - 1, (arr.len() / 2) + 1)
    );
}
