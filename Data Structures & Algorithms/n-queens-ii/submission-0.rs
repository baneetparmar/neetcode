impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1];
        let mut diag2 = vec![false; 2 * n - 1];

        fn backtrack(cols: &mut Vec<bool>, diag1: &mut Vec<bool>, diag2: &mut Vec<bool>, row: usize, n: usize) -> i32 {
            if row == n { return 1; }
            let mut count = 0;
            for col in 0..n {
                let d1 = row + n - 1 - col;
                let d2 = row + col;
                if cols[col] || diag1[d1] || diag2[d2] { continue; }
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                count += backtrack(cols, diag1, diag2, row + 1, n);
                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
            count
        }

        backtrack(&mut cols, &mut diag1, &mut diag2, 0, n)
    }
}