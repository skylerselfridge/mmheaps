use std::cmp::Ordering;

pub struct HeapNode<T> {
    pub data: T,
    pub priority: i32,
}

impl<T> PartialOrd for HeapNode<T> {
    fn partial_cmp(&self, other: &HeapNode<T>) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<T> PartialEq for HeapNode<T> {
    fn eq(&self, other: &HeapNode<T>) -> bool {
        self.priority == other.priority
    }
}
/// Creates an iterable MinHeap with type T
///
/// # Examples
///
/// ```
/// let min_heap = MinHeap::<i32>::new();
/// min_heap.push(3,2);
/// min_heap.push(2,1);
/// min_heap.push(1,0);
/// min_heap.push(0,-1);
/// for i in min_heap {
///    println("{:?}",i);
/// }
/// ```
pub struct MinHeap<T> {
    heap: Vec<HeapNode<T>>,
}

impl<T> MinHeap<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn push(&mut self, data: T, priority: i32) {
        let node = HeapNode { data, priority };
        self.heap.push(node);
        let len = self.heap.len();
        self.bubble_up(len - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.len() {
            0 => None,
            1 => self.heap.pop().map(|node| node.data),
            _ => {
                let last_index = self.heap.len() - 1;
                self.heap.swap(0, last_index);
                let node = self.heap.pop().unwrap();
                self.bubble_down(0);
                Some(node.data)
            }
        }
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.heap[index] > self.heap[parent_index] {
                self.heap.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let left_child_index = index * 2 + 1;
            let right_child_index = index * 2 + 2;
            let mut smallest_index = index;

            if left_child_index < self.heap.len()
                && self.heap[left_child_index] > self.heap[smallest_index]
            {
                smallest_index = left_child_index;
            }

            if right_child_index < self.heap.len()
                && self.heap[right_child_index] > self.heap[smallest_index]
            {
                smallest_index = right_child_index;
            }

            if smallest_index != index {
                self.heap.swap(index, smallest_index);
                index = smallest_index;
            } else {
                break;
            }
        }
    }
}

impl<T: Ord> IntoIterator for MinHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { heap: self }
    }
}

pub struct IntoIter<T: Ord> {
    heap: MinHeap<T>,
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}
