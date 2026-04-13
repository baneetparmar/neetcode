use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        trips.sort_by_key(|t| t[1]);
        let mut filled = 0;
        let mut min_heap = BinaryHeap::new();
        for trip in trips {
            let ppl = trip[0];
            let bo = trip[1];
            let de = trip[2];
            while let Some(&(Reverse(d), p)) = min_heap.peek() {
                if d <= bo {
                    filled -= p;
                } else {
                    break;
                }
                min_heap.pop();
            }

            filled += ppl;

            if filled > capacity {
                return false;
            }
            min_heap.push((Reverse(de), ppl));
        }

        true
    }
}
