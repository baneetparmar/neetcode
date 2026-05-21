impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent: Vec<usize> = (0..=n).collect();
        let mut rank = vec![0usize; n + 1];

        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            let ra = Self::find(&mut parent, a);
            let rb = Self::find(&mut parent, b);
            if ra == rb { return vec![e[0], e[1]]; }
            if rank[ra] < rank[rb] { parent[ra] = rb; }
            else if rank[ra] > rank[rb] { parent[rb] = ra; }
            else { parent[rb] = ra; rank[ra] += 1; }
        }

        vec![]
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }
}