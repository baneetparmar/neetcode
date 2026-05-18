impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len() != n - 1 { return false; }

        let mut adj = vec![vec![]; n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
        }

        let mut visited = vec![false; n];
        let mut q = std::collections::VecDeque::from([0]);
        visited[0] = true;
        let mut count = 1;

        while let Some(node) = q.pop_front() {
            for &next in &adj[node] {
                if !visited[next] {
                    visited[next] = true;
                    count += 1;
                    q.push_back(next);
                }
            }
        }

        count == n
    }
}