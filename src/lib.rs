use std::fmt::Debug;

pub struct PriorityQueue<T> {
    pub queue: Vec<Node<T>>,
    pub heap_size: usize,
}
#[derive(Copy, Clone, Debug)]
pub struct Node<T> {
    pub key: i32,
    pub value: T,
}

impl<T: Clone + Copy + Debug> PriorityQueue<T> {
    pub fn new(&mut self, init_val: Node<T>) {
        self.queue = vec![init_val];
        self.heap_size = self.queue.len();
    }

    pub fn insert(&mut self, value: Node<T>) {
        self.heap_size += 1;
        self.queue.push(value);
        let last_key = self.queue[self.heap_size - 1].key;
        self.increase_key(&last_key, self.heap_size - 1);
    }

    pub fn maximum(&self) -> &T {
        return &self.queue[0].value;
    }

    fn max_heapify(&mut self, idx: usize) {
        if self.heap_size == 0 {
            return;
        }
        let left = 2 * idx + 1;
        let right = left + 1;

        let mut largest: usize;

        if left <= self.heap_size - 1 && self.queue[left].key >= self.queue[idx].key {
            largest = left;
        } else {
            largest = idx;
        }

        if right <= largest && self.queue[right].key >= self.queue[largest].key {
            largest = right;
        }

        if largest != idx {
            self.queue.swap(idx, largest);
            self.max_heapify(largest);
        }
    }

    pub fn increase_key(&mut self, key: &i32, mut idx: usize) {
        if !*key < self.queue[idx].key {
            self.queue[idx].key = *key;
            let root_idx = (idx - 1) / 2;
            while idx > 0 && self.queue[root_idx].key < self.queue[idx].key {
                self.queue.swap(idx, root_idx);
                idx = root_idx;
            }
        }
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.heap_size > 0 {
            let node = self.queue.remove(0);
            self.heap_size = self.heap_size - 1;

            self.max_heapify(0 as usize);
            return Some(node.value);
        } else {
            return None;
        }
    }
}
