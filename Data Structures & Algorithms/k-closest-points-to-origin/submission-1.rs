use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
        for point in points {
            let x = point[0];
            let y = point[1];
            let distance = x*x + y*y;
            let tup = (distance, x, y);
            heap.push(tup);
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_,x,y)| vec![x,y]).collect()
    }
}
