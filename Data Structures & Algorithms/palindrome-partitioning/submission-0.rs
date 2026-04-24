impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s: Vec<char> = s.chars().collect();
        let mut result = vec![];
        let mut current = vec![];

        fn is_palindrome(s: &[char], l: usize, r: usize) -> bool {
            let (mut l, mut r) = (l, r);
            while l < r {
                if s[l] != s[r] { return false; }
                l += 1;
                r -= 1;
            }
            true
        }

        fn backtrack(s: &[char], start: usize, cur: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if start == s.len() {
                res.push(cur.clone());
                return;
            }
            for end in start..s.len() {
                if is_palindrome(s, start, end) {
                    cur.push(s[start..=end].iter().collect());
                    backtrack(s, end + 1, cur, res);
                    cur.pop();
                }
            }
        }

        backtrack(&s, 0, &mut current, &mut result);
        result
    }
}