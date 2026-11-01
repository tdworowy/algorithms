//! A simple, unbalanced binary search tree.
//!
//! Values must implement [`Ord`]. Inserting a value already in the tree is a
//! no-op and returns `false`.

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

/// A set of values stored in binary search tree order.
///
/// Operations take `O(h)` time, where `h` is the tree height. Since this tree
/// does not rebalance itself, `h` can grow to `n` for unfavourable insertion
/// orders.
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Link<T>,
    len: usize,
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns the number of values in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` when the tree contains no values.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts `value`. Returns `true` if it was added, or `false` if an equal
    /// value was already present.
    pub fn insert(&mut self, value: T) -> bool {
        let mut link = &mut self.root;

        loop {
            match link {
                Some(node) => match value.cmp(&node.value) {
                    std::cmp::Ordering::Less => link = &mut node.left,
                    std::cmp::Ordering::Greater => link = &mut node.right,
                    std::cmp::Ordering::Equal => return false,
                },
                None => {
                    *link = Some(Box::new(Node::new(value)));
                    self.len += 1;
                    return true;
                }
            }
        }
    }

    /// Returns whether an equal value is in the tree.
    pub fn contains(&self, value: &T) -> bool {
        let mut current = self.root.as_deref();

        while let Some(node) = current {
            match value.cmp(&node.value) {
                std::cmp::Ordering::Less => current = node.left.as_deref(),
                std::cmp::Ordering::Greater => current = node.right.as_deref(),
                std::cmp::Ordering::Equal => return true,
            }
        }

        false
    }

    /// Removes `value`. Returns `true` if it was present.
    pub fn remove(&mut self, value: &T) -> bool {
        let mut link = &mut self.root;

        loop {
            // Compare while the node is still in the tree. If a user-defined
            // `Ord::cmp` panics, no part of the tree has been detached.
            let ordering = match link.as_ref() {
                Some(node) => value.cmp(&node.value),
                None => return false,
            };

            match ordering {
                std::cmp::Ordering::Less => {
                    link = &mut link.as_mut().expect("node was just inspected").left;
                }
                std::cmp::Ordering::Greater => {
                    link = &mut link.as_mut().expect("node was just inspected").right;
                }
                std::cmp::Ordering::Equal => {
                    let mut node = link.take().expect("node was just inspected");
                    *link = match (node.left.take(), node.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(right)) => {
                            let (remaining_right, mut successor) = take_min(right);
                            successor.left = Some(left);
                            successor.right = remaining_right;
                            Some(successor)
                        }
                    };
                    self.len -= 1;
                    return true;
                }
            }
        }
    }

    /// Removes and returns the smallest value, or `None` if the tree is empty.
    pub fn take_min(&mut self) -> Option<T> {
        let root = self.root.take()?;
        let (remaining_root, minimum) = take_min(root);

        self.root = remaining_root;
        self.len -= 1;
        Some(minimum.value)
    }

    /// Removes and returns the largest value, or `None` if the tree is empty.
    pub fn take_max(&mut self) -> Option<T> {
        let mut link = &mut self.root;

        loop {
            let has_right_child = match link.as_ref() {
                Some(node) => node.right.is_some(),
                None => return None,
            };

            if has_right_child {
                link = &mut link.as_mut().expect("node was just inspected").right;
            } else {
                let mut maximum = link.take().expect("node was just inspected");
                *link = maximum.left.take();
                self.len -= 1;
                return Some(maximum.value);
            }
        }
    }

    /// Visits values in ascending order.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.root.as_deref())
    }
}

/// Removes and returns the minimum node, along with the remaining subtree.
fn take_min<T>(mut node: Box<Node<T>>) -> (Link<T>, Box<Node<T>>) {
    if node.left.is_none() {
        return (node.right.take(), node);
    }

    let mut parent = &mut node;
    while parent
        .left
        .as_ref()
        .is_some_and(|left_child| left_child.left.is_some())
    {
        parent = parent.left.as_mut().expect("left child was just inspected");
    }

    let mut minimum = parent.left.take().expect("left child was just inspected");
    parent.left = minimum.right.take();
    (Some(node), minimum)
}

impl<T> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        // Detach each node's children before dropping it so a skewed tree does
        // not recursively destroy a long chain of `Box<Node<T>>` values.
        let mut pending = Vec::new();
        if let Some(root) = self.root.take() {
            pending.push(root);
        }

        while let Some(mut node) = pending.pop() {
            if let Some(left) = node.left.take() {
                pending.push(left);
            }
            if let Some(right) = node.right.take() {
                pending.push(right);
            }
        }
    }
}

/// An in-order iterator over a [`BinarySearchTree`].
pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    fn new(root: Option<&'a Node<T>>) -> Self {
        let mut iter = Self { stack: Vec::new() };
        iter.push_left_spine(root);
        iter
    }

    fn push_left_spine(&mut self, mut node: Option<&'a Node<T>>) {
        while let Some(current) = node {
            self.stack.push(current);
            node = current.left.as_deref();
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.push_left_spine(node.right.as_deref());
        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{BinarySearchTree, Link, Node};
    use std::cell::Cell;
    use std::cmp::Ordering;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::rc::Rc;

    #[test]
    fn insert_contains_and_len_handle_duplicates() {
        let mut tree = BinarySearchTree::new();

        assert!(tree.is_empty());
        assert!(tree.insert(5));
        assert!(tree.insert(2));
        assert!(tree.insert(8));
        assert!(!tree.insert(5));

        assert_eq!(tree.len(), 3);
        assert!(!tree.is_empty());
        assert!(tree.contains(&2));
        assert!(tree.contains(&5));
        assert!(tree.contains(&8));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn iter_returns_values_in_sorted_order() {
        let mut tree = BinarySearchTree::new();
        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        assert_eq!(
            tree.iter().copied().collect::<Vec<_>>(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn take_max_returns_largest_value_and_updates_tree() {
        let mut tree = BinarySearchTree::new();

        assert_eq!(tree.take_max(), None);

        for value in [8, 3, 10, 1, 6, 14, 13] {
            tree.insert(value);
        }

        assert_eq!(tree.take_max(), Some(14));
        assert_eq!(tree.len(), 6);
        assert_eq!(
            tree.iter().copied().collect::<Vec<_>>(),
            vec![1, 3, 6, 8, 10, 13]
        );
        assert_eq!(tree.take_max(), Some(13));
        assert_eq!(tree.take_max(), Some(10));
        assert_eq!(tree.take_max(), Some(8));
        assert_eq!(tree.take_max(), Some(6));
        assert_eq!(tree.take_max(), Some(3));
        assert_eq!(tree.take_max(), Some(1));
        assert_eq!(tree.take_max(), None);
        assert!(tree.is_empty());
    }

    #[test]
    fn take_min_returns_smallest_value_and_preserves_right_subtree() {
        let mut tree = BinarySearchTree::new();

        assert_eq!(tree.take_min(), None);

        for value in [8, 4, 12, 1, 3, 2, 6] {
            tree.insert(value);
        }

        assert_eq!(tree.take_min(), Some(1));
        assert_eq!(tree.len(), 6);
        assert_eq!(
            tree.iter().copied().collect::<Vec<_>>(),
            vec![2, 3, 4, 6, 8, 12]
        );
        assert_eq!(tree.take_min(), Some(2));
        assert_eq!(tree.take_min(), Some(3));
        assert_eq!(tree.take_min(), Some(4));
        assert_eq!(tree.take_min(), Some(6));
        assert_eq!(tree.take_min(), Some(8));
        assert_eq!(tree.take_min(), Some(12));
        assert_eq!(tree.take_min(), None);
        assert!(tree.is_empty());
    }

    #[test]
    fn remove_handles_leaf_one_child_and_two_child_nodes() {
        let mut tree = BinarySearchTree::new();
        for value in [20, 10, 30, 25, 27, 40] {
            tree.insert(value);
        }

        assert!(tree.remove(&20)); // Two children; successor 25 has right child 27.
        assert!(tree.remove(&10)); // Leaf.
        assert!(tree.remove(&40)); // Leaf.
        assert!(tree.remove(&30)); // One child after removing 40.
        assert!(!tree.remove(&99));

        assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![25, 27]);
        assert_eq!(tree.len(), 2);
    }

    #[derive(Clone)]
    struct PanicOnCompare {
        key: i32,
        panic_next_compare: Rc<Cell<bool>>,
    }

    impl PartialEq for PanicOnCompare {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl Eq for PanicOnCompare {}

    impl PartialOrd for PanicOnCompare {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for PanicOnCompare {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.panic_next_compare.replace(false) {
                panic!("comparison panic requested by test");
            }
            self.key.cmp(&other.key)
        }
    }

    #[test]
    fn remove_preserves_tree_if_comparison_panics() {
        let panic_next_compare = Rc::new(Cell::new(false));
        let mut tree = BinarySearchTree::new();
        for key in [1, 2, 3] {
            tree.insert(PanicOnCompare {
                key,
                panic_next_compare: Rc::clone(&panic_next_compare),
            });
        }

        panic_next_compare.set(true);
        let probe = PanicOnCompare {
            key: 2,
            panic_next_compare: Rc::clone(&panic_next_compare),
        };
        let result = catch_unwind(AssertUnwindSafe(|| tree.remove(&probe)));

        assert!(result.is_err());
        assert_eq!(tree.len(), 3);
        assert_eq!(
            tree.iter().map(|value| value.key).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn deeply_skewed_tree_can_be_searched_removed_and_dropped() {
        const DEPTH: i32 = 50_000;

        let mut root: Link<i32> = None;
        let mut link = &mut root;
        for value in 0..DEPTH {
            *link = Some(Box::new(Node::new(value)));
            link = &mut link.as_mut().unwrap().right;
        }

        let mut tree = BinarySearchTree {
            root,
            len: DEPTH as usize,
        };
        assert!(tree.contains(&(DEPTH - 1)));
        assert_eq!(tree.take_max(), Some(DEPTH - 1));
        assert_eq!(tree.len(), DEPTH as usize - 1);
        drop(tree);
    }
}
