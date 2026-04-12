use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counter = HashMap::new();

        for ch in s.chars() {
            *counter.entry(ch).or_insert(0) += 1;
        }

        let mut choices = BinaryHeap::new();
        for (k, v) in counter {
            choices.push((v, k));
        }

        let mut out = String::new();
        let mut prev = None;
        while let Some((freq, ch)) = choices.pop() {
            out.push(ch);

            if let Some((f, c)) = prev {
                if f > 0 {
                    choices.push((f, c));
                }
            }

            prev = Some((freq - 1, ch));
        }

        if s.len() == out.len() {
            out
        } else {
            "".to_owned()
        }
    }
}
