/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::mem::replace;




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
            items: Vec::new(),
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
        self.count+=1;
        let mut ind=self.count-1;
        while ind!=0&&(self.comparator)(&self.items[ind],&self.items[self.parent_idx(ind)]){
            self.swap(ind,self.parent_idx(ind));
            ind=self.parent_idx(ind);
        }

    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx-1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2+1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);  
        let right = self.right_child_idx(idx);  
        if left >= self.count {  
            return idx; 
        }  
        let smallest = if right >= self.count {  
            left  
        } else {  
            if (self.comparator)(&self.items[left], &self.items[right]) {  
                left  
            } else {  
                right  
            }  
        };  
        if (self.comparator)(&self.items[idx], &self.items[smallest]) {  
            idx  
        } else {  
            smallest  
        }  
    }

    fn heapify_down(&mut self, idx: usize) {  

        let smallest = self.smallest_child_idx(idx);  

        if smallest != idx {  

            self.swap(idx, smallest);  

            self.heapify_down(smallest);  

        }  

    }  

  

    fn swap(&mut self, i: usize, j: usize) {  

        self.items.swap(i, j);  

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

T: Ord + Copy+ std::default::Default



{  

    type Item = T;  

  

    fn next(&mut self) -> Option<T> {  

        if self.is_empty() {  

            return None;  

        }  

        let last_index = self.count as usize - 1;  

        self.items.swap(0, last_index);  

        self.count -= 1;  

        self.heapify_down(0);  

        Some(self.items.pop().unwrap())  

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