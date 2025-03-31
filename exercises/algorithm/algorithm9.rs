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

    fn bubble_up(&mut self, mut idx: usize) {
        // 循环 检查当前节点是否比父节点更符合堆的顺序：
        // 若是，则交换它们的位置。
        // 若不是，则停止调整。
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /**
     * 当 next 取出堆顶元素后，用最后一个元素填补位置，此时可能破坏堆的顺序，需要下滤修正。
     * 检查当前节点是否大于/小于其子节点：
     * 若是，则交换它与最优子节点（对于最小堆是较小的子节点，对于最大堆是较大的子节点）。
     * 若不是，则停止调整。
     */
    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest = self.smallest_child_idx(idx);
            if smallest >= self.count {
                break;
            }
            if (self.comparator)(&self.items[smallest], &self.items[idx]) {
                self.items.swap(idx, smallest);
                idx = smallest;
            } else {
                break;
            }
        }
    }

    /**
     * 将元素 value 添加到堆的 items 数组末尾。
     * 递增 count 以记录元素个数。
     * 调用 bubble_up 将新插入的元素移动到正确位置，确保堆性质不被破坏。
     */
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.bubble_up(self.count);
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

    // 首先获取左子节点 left 和右子节点 right 的索引。
    // 检查右子节点是否存在：
    // 若 不存在，直接返回 left。
    // 若 存在，比较 left 和 right，返回符合堆性质的较优者。
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right >= self.count || (self.comparator)(&self.items[left], &self.items[right]) {
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
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 如果堆为空，返回 None
        // 取出堆顶元素，并用最后一个元素替代堆顶。
        // 调用 bubble_down 修正堆，使得新的堆顶仍然是正确的最小/最大值。
        // 返回原堆顶元素。
        if self.count == 0 {
            return None;
        }
        
        let top = self.items[1].clone();
        if self.count > 1 {
            self.items[1] = self.items.pop().unwrap();  // 用最后一个元素替代堆顶
            self.bubble_down(1);    // 调整堆
        } else {
            self.items.pop();
        }
        self.count -= 1;
        Some(top)
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