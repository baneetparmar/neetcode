impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = vec![];
        let mut board = vec![vec!['.'; n]; n];
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1];
        let mut diag2 = vec![false; 2 * n - 1];

        fn backtrack(
            board: &mut Vec<Vec<char>>,
            cols: &mut Vec<bool>,
            diag1: &mut Vec<bool>,
            diag2: &mut Vec<bool>,
            row: usize,
            n: usize,
            res: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                res.push(board.iter().map(|r| r.iter().collect()).collect());
                return;
            }
            for col in 0..n {
                let d1 = row + n - 1 - col;
                let d2 = row + col;
                if cols[col] || diag1[d1] || diag2[d2] { continue; }
                board[row][col] = 'Q';
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                backtrack(board, cols, diag1, diag2, row + 1, n, res);
                board[row][col] = '.';
                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }

        backtrack(&mut board, &mut cols, &mut diag1, &mut diag2, 0, n, &mut result);
        result
    }
}