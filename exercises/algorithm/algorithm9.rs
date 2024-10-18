use std::cmp::Ord;
use std::default::Default;
use std::clone::Clone;

pub struct Heap<T>
where
    T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        self.heapify_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if!self.children_present(idx) {
            return idx;
        }
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        if right_idx > self.count {
            return left_idx;
        }
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            return left_idx;
        } else {
            return right_idx;
        }
    }

    fn heapify_up(&mut self, idx: usize) {
        let mut current_idx = idx;
        while current_idx > 1 && (self.comparator)(&self.items[current_idx], &self.items[self.parent_idx(current_idx)]) {
            let parent_idx = self.parent_idx(current_idx);
            let current_item = self.items[current_idx].clone();
            let parent_item = self.items[parent_idx].clone();
            self.items[current_idx] = parent_item;
            self.items[parent_idx] = current_item;
            current_idx = parent_idx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Clone + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min = self.items[1].clone();
        self.items[1] = self.items[self.count].clone();
        self.count -= 1;
        // self.heapify_down(1);
        Some(min)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Clone + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Clone + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}