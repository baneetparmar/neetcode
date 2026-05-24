use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 { return vec![0]; }

        let mut adj = vec![vec![]; n];
        let mut degree = vec![0usize; n];

        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
            degree[a] += 1;
            degree[b] += 1;
        }

        let mut q: VecDeque<usize> = degree
            .iter()
            .enumerate()
            .filter(|&(_, &d)| d == 1)
            .map(|(i, _)| i)
            .collect();

        let mut remaining = n;
        while remaining > 2 {
            let leaves = q.len();
            remaining -= leaves;
            for _ in 0..leaves {
                let leaf = q.pop_front().unwrap();
                for &next in &adj[leaf] {
                    degree[next] -= 1;
                    if degree[next] == 1 {
                        q.push_back(next);
                    }
                }
            }
        }

        q.into_iter().map(|i| i as i32).collect()
    }
}