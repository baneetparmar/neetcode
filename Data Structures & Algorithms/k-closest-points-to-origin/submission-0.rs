use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
        for point in points {
            let x = point[0];
            let y = point[1];
            let distance = Self::distance_from_origin(x, y);
            let tup = (distance, x, y);
            if heap.len() == k {
                if let Some(r) = heap.peek() {
                    if r.0 > tup.0 {
                        heap.pop();
                        heap.push(tup);
                    }
                }
            } else {
                heap.push(tup);
            }
        }
        let mut res = vec![];
        while let Some(tup) = heap.pop() {
            let point = vec![tup.1, tup.2];
            res.push(point);
        }
        res
    }
    fn distance_from_origin(x: i32, y: i32) -> i32 {
        x * x + y * y
    }
}
