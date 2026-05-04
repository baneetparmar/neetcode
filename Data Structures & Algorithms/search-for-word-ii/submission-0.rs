impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let rows = board.len();
        let cols = board[0].len();
        let mut board = board;
        let mut result = vec![];

        let mut trie: Vec<[i32; 26]> = vec![[-1; 26]];
        let mut is_end: Vec<Option<String>> = vec![None];

        for word in words {
            let mut node = 0;
            for c in word.bytes().map(|b| (b - b'a') as usize) {
                if trie[node][c] == -1 {
                    trie[node][c] = trie.len() as i32;
                    trie.push([-1; 26]);
                    is_end.push(None);
                }
                node = trie[node][c] as usize;
            }
            is_end[node] = Some(word);
        }

        fn backtrack(
            board: &mut Vec<Vec<char>>,
            trie: &Vec<[i32; 26]>,
            is_end: &mut Vec<Option<String>>,
            node: usize,
            row: usize,
            col: usize,
            result: &mut Vec<String>,
        ) {
            let c = board[row][col];
            if c == '#' { return; }
            let idx = (c as u8 - b'a') as usize;
            let next = trie[node][idx];
            if next == -1 { return; } 
            let next = next as usize;
            if let Some(word) = is_end[next].take() {
                result.push(word);
            }
            board[row][col] = '#';
            let dirs = [(!0, 0), (1, 0), (0, !0usize), (0, 1)];
            for (dr, dc) in dirs {
                let r = row.wrapping_add(dr);
                let c2 = col.wrapping_add(dc);
                if r < board.len() && c2 < board[0].len() {
                    backtrack(board, trie, is_end, next, r, c2, result);
                }
            }
            board[row][col] = c;
        }

        for r in 0..rows {
            for c in 0..cols {
                backtrack(&mut board, &trie, &mut is_end, 0, r, c, &mut result);
            }
        }
        result
    }
}