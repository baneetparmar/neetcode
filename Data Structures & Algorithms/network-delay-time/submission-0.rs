use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n + 1];
        for t in &times {
            adj[t[0] as usize].push((t[1] as usize, t[2]));
        }

        let mut dist = vec![i32::MAX; n + 1];
        let mut heap = BinaryHeap::new();
        dist[k as usize] = 0;
        heap.push(Reverse((0, k as usize)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] { continue; }
            for &(v, w) in &adj[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }

        let ans = dist[1..=n].iter().max().unwrap();
        if *ans == i32::MAX { -1 } else { *ans }
    }
}