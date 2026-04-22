impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut current = String::new();

        fn backtrack(open: i32, close: i32, n: i32, cur: &mut String, res: &mut Vec<String>) {
            if cur.len() == (2 * n) as usize {
                res.push(cur.clone());
                return;
            }
            if open < n {
                cur.push('(');
                backtrack(open + 1, close, n, cur, res);
                cur.pop();
            }
            if close < open {
                cur.push(')');
                backtrack(open, close + 1, n, cur, res);
                cur.pop();
            }
        }

        backtrack(0, 0, n, &mut current, &mut result);
        result
    }
}