use std::{cmp::Reverse, collections::BinaryHeap};

pub struct KthLargest {
    k: usize,
    scores: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();

        for num in nums {
            heap.push(Reverse(num));
        }

        while heap.len() > k {
            heap.pop();
        }

        Self { k, scores: heap }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.k == self.scores.len() {
            if let Some(Reverse(top_val)) = self.scores.pop() {
                if top_val >= val {
                    self.scores.push(Reverse(top_val));
                    return top_val;
                }
            }
        }

        self.scores.push(Reverse(val));
        if let Some(&Reverse(val)) = self.scores.peek() {
            return val;
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);

        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }

    #[test]
    fn test_case_2() {
        let mut kth_largest = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);

        assert_eq!(kth_largest.add(2), 7);
        assert_eq!(kth_largest.add(10), 7);
        assert_eq!(kth_largest.add(9), 7);
        assert_eq!(kth_largest.add(9), 8);
    }
}
