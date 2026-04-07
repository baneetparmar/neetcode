use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let s1 = heap.pop().unwrap();
            let s2 = heap.pop().unwrap();
            let new_s = (s2 - s1).abs();
            if new_s > 0 {
                heap.push(new_s);
            }
        }
        match heap.pop() {
            Some(v) => v,
            None => 0
        }
    }
}
