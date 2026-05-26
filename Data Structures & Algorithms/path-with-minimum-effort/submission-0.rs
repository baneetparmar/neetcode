use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (heights.len(), heights[0].len());
        let mut effort = vec![vec![i32::MAX; cols]; rows];
        let mut heap = BinaryHeap::new();

        effort[0][0] = 0;
        heap.push(Reverse((0, 0, 0)));

        while let Some(Reverse((eff, r, c))) = heap.pop() {
            if r == rows - 1 && c == cols - 1 { return eff; }
            if eff > effort[r][c] { continue; }

            for (nr, nc) in [
                (r.wrapping_sub(1), c), (r+1, c),
                (r, c.wrapping_sub(1)), (r, c+1)
            ] {
                if nr < rows && nc < cols {
                    let next_eff = eff.max((heights[nr][nc] - heights[r][c]).abs());
                    if next_eff < effort[nr][nc] {
                        effort[nr][nc] = next_eff;
                        heap.push(Reverse((next_eff, nr, nc)));
                    }
                }
            }
        }
        0
    }
}