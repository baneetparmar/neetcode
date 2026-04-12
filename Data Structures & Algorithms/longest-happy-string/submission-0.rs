use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules! push_if_positive {
    ($heap:expr, $count:expr, $ch:expr) => {
        if $count > 0 {
            $heap.push(($count, $ch));
        }
    };
}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut out = vec![];
        let mut heap = BinaryHeap::new();
        push_if_positive!(heap, a, b'a');
        push_if_positive!(heap, b, b'b');
        push_if_positive!(heap, c, b'c');

        while let Some((count, ch)) = heap.pop() {
            if out.len() > 1 && out[out.len() - 1] == ch && out[out.len() - 2] == ch {
                if let Some((count2, ch2)) = heap.pop() {
                    out.push(ch2);
                    if count2 - 1 > 0 {
                        heap.push((count2 - 1, ch2));
                    }
                    heap.push((count, ch));
                } else {
                    break;
                }
            } else {
                out.push(ch);
                if count - 1 > 0 {
                    heap.push((count - 1, ch));
                }
            }
        }

        String::from_utf8(out).unwrap()
    }
}
