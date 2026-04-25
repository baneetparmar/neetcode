impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() { return vec![]; }

        let map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result = vec![];
        let mut current = String::new();
        let digits: Vec<usize> = digits.chars().map(|c| c as usize - '2' as usize).collect();

        fn backtrack(digits: &[usize], map: &[&str], idx: usize, cur: &mut String, res: &mut Vec<String>) {
            if idx == digits.len() {
                res.push(cur.clone());
                return;
            }
            for ch in map[digits[idx]].chars() {
                cur.push(ch);
                backtrack(digits, map, idx + 1, cur, res);
                cur.pop();
            }
        }

        backtrack(&digits, &map, 0, &mut current, &mut result);
        result
    }
}