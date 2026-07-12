#[derive(Debug)]
pub struct MaxPriorityQueue<T> {
    heap: Vec<T>,
    heap_size: usize,
}

impl<T: Ord> MaxPriorityQueue<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new(),  heap_size: 0, }
    }

    pub fn from_vec(mut heap: Vec<T>) -> Self {
        let heap_size = heap.len();
        build_max_heap(&mut heap, heap_size);
        Self { heap, heap_size }
    }

    pub fn maximum(&self) -> Option<&T> {
        if self.heap_size == 0 {
            None
        } else {
            Some(&self.heap[0])
        }
    }

    pub fn extract_max(&mut self) -> Option<&T> {
        if self.heap_size == 0 {
            return None;
        }

        self.heap.swap(0, self.heap_size -1);
        self.heap_size -= 1;

        if self.heap_size > 0 {
            max_heapify(&mut self.heap, 0, self.heap_size);
        }
        Some(&self.heap[self.heap_size])

    }

    pub fn increase_key(&mut self, mut i: usize, key: T) -> Result<(), &'static str> {
        if i >= self.heap.len() {
            return Err("index out of bounds");
        }

        if key < self.heap[i] {
            return Err("new key is smaller than current key");
        }

        self.heap[i] = key;

        while i > 0 && self.heap[parent(i)] < self.heap[i] {
            let p = parent(i);
            self.heap.swap(i, p);
            i = p;
        }

        Ok(())
    }

    pub fn insert(&mut self, key: T) {
        if self.heap_size < self.heap.len() {
            self.heap[self.heap_size] = key;
        } else {
            self.heap.push(key);
        }
        self.heap_size += 1;

        let mut i = self.heap_size - 1;

        while i > 0 && self.heap[parent(i)] < self.heap[i] {
            let p = parent(i);
            self.heap.swap(i, p);
            i = p;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

fn max_heapify<T: Ord>(heap: &mut [T], i: usize,  heap_size: usize,) {
    let left = left(i);
    let right = right(i);

    let mut largest = i;

    if left < heap_size && heap[left] > heap[largest] {
        largest = left;
    }

    if right <heap_size && heap[right] > heap[largest] {
        largest = right;
    }

    if largest != i {
        heap.swap(i, largest);
        max_heapify(heap, largest, heap_size);
    }
}

fn build_max_heap<T: Ord>(heap: &mut [T],  heap_size: usize,) {

    for i in (0..heap.len() / 2).rev() {
        max_heapify(heap, i, heap_size);
    }
}

impl<T: Ord> Default for MaxPriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum() {
        let queue = MaxPriorityQueue::from_vec(vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);

        assert_eq!(queue.maximum(), Some(&16));
    }

    #[test]
    fn extract_max() {
        let mut queue = MaxPriorityQueue::from_vec(vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);

        assert_eq!(queue.extract_max(), Some(&16));
        assert_eq!(queue.extract_max(), Some(&14));
        assert_eq!(queue.extract_max(), Some(&10));
        assert_eq!(queue.extract_max(), Some(&9));
    }

    #[test]
    fn insert() {
        let mut queue = MaxPriorityQueue::new();

        queue.insert(4);
        queue.insert(1);
        queue.insert(16);
        queue.insert(8);
        queue.insert(10);

        assert_eq!(queue.maximum(), Some(&16));
    }

    #[test]
    fn increase_key() {
        let mut queue = MaxPriorityQueue::from_vec(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);

        queue.increase_key(8, 15).unwrap();

        assert_eq!(queue.maximum(), Some(&16));
        assert_eq!(queue.extract_max(), Some(&16));
        assert_eq!(queue.extract_max(), Some(&15));
    }

    #[test]
    fn extract_all() {
        let mut queue = MaxPriorityQueue::new();

        for value in [5, 3, 17, 10, 84, 19, 6, 22, 9] {
            queue.insert(value);
        }

        let mut result = Vec::new();

        while let Some(max) = queue.extract_max() {
            result.push(*max);
        }

        assert_eq!(result, vec![84, 22, 19, 17, 10, 9, 6, 5, 3]);
    }
}
fn main() {
    println!("Hello, world!");
}
