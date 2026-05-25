use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set: HashSet<&str> = word_list.iter().map(|s| s.as_str()).collect();
        if !word_set.contains(end_word.as_str()) { return 0; }

        let mut visited: HashSet<&str> = HashSet::new();
        let mut q = VecDeque::from([(begin_word.as_str(), 1)]);
        visited.insert(begin_word.as_str());

        while let Some((word, steps)) = q.pop_front() {
            let chars: Vec<u8> = word.bytes().collect();
            for i in 0..chars.len() {
                for c in b'a'..=b'z' {
                    if c == chars[i] { continue; }
                    let mut next = chars.clone();
                    next[i] = c;
                    let next_str = std::str::from_utf8(&next).unwrap();
                    if next_str == end_word { return steps + 1; }
                    if word_set.contains(next_str) && !visited.contains(next_str) {
                        visited.insert(word_set.get(next_str).unwrap());
                        q.push_back((word_set.get(next_str).unwrap(), steps + 1));
                    }
                }
            }
        }
        0
    }
}