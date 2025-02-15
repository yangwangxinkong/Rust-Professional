/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
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
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut current_idx = self.items.len() - 1;
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;
            } else {
                break;
            }
        }
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
        //TODO
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right > self.count {
            left
        } else if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default + std::fmt::Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            return None;
        }
        let result = self.items.swap_remove(1);
        println!("swap_remove {}", &result);
        self.count -= 1;
        if self.is_empty() {
            println!("返回结果：{}", result);
            return Some(result);
        }
        self.println("items.pop 前 ");
        let t = self.items.pop().unwrap();
        println!("items.pop {}", &t);
        self.items.insert(1, t);
        self.println("items.pop 后 ");
        let mut current_idx = 1;
        // 判断有没有子节点
        while self.children_present(current_idx) {
            let smallest_child_idx = self.smallest_child_idx(current_idx);
            println!("current_idx {} smallest_child_idx {}",current_idx, smallest_child_idx);
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[current_idx]) {
                self.items.swap(smallest_child_idx, current_idx);
                current_idx = smallest_child_idx;
            } else {
                break;
            }
        }
        self.println("返回结果前： ");
        Some(result)
    }
}

impl<T> Heap<T>
where
    T: Default + std::fmt::Display,
{
    fn println(&mut self, info: &str) {
        print!("{}", info);
        for x in &self.items {
            print!("items {} ", x);
        }
        println!();
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
