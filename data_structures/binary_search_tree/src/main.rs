use binary_search_tree::BinarySearchTree;

fn main() {
    let mut tree = BinarySearchTree::new();
    for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
        tree.insert(value);
    }

    println!(
        "sorted values: {:?}",
        tree.iter().copied().collect::<Vec<_>>()
    );
    println!("contains 6: {}", tree.contains(&6));

    tree.remove(&3);
    println!(
        "after removing 3: {:?}",
        tree.iter().copied().collect::<Vec<_>>()
    );
}
