impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let dict: HashSet<String> = dictionary.into_iter().collect();
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp = vec![0i32; n + 1];

        for i in 1..=n {
            dp[i] = dp[i - 1] + 1;
            for j in 0..i {
                let word: String = s[j..i].iter().collect();
                if dict.contains(&word) {
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }

        dp[n]
    }
}