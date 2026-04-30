impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let s: Vec<char> = s.chars().collect();
        let mut result = vec![];
        let mut current = vec![];

        fn backtrack(s: &[char], dict: &HashSet<String>, start: usize, cur: &mut Vec<String>, res: &mut Vec<String>) {
            if start == s.len() {
                res.push(cur.join(" "));
                return;
            }
            for end in start..s.len() {
                let word: String = s[start..=end].iter().collect();
                if dict.contains(&word) {
                    cur.push(word);
                    backtrack(s, dict, end + 1, cur, res);
                    cur.pop();
                }
            }
        }

        backtrack(&s, &dict, 0, &mut current, &mut result);
        result
    }
}