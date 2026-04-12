use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut output = vec![];
        let mut indexed_tasks: Vec<(i32, i32, usize)> = tasks
            .iter()
            .enumerate()
            .map(|(i, t)| (t[0], t[1], i))
            .collect();
        indexed_tasks.sort();

        let mut t = 0;
        let mut i = 0;
        let mut ready = BinaryHeap::new();

        while output.len() < tasks.len() {
            while i < indexed_tasks.len() && indexed_tasks[i].0 <= t {
                ready.push(Reverse((indexed_tasks[i].1, indexed_tasks[i].2)));
                i += 1;
            }

            if ready.is_empty() {
                t = indexed_tasks[i].0;
                continue;
            }
            let Reverse((pt, idx)) = ready.pop().unwrap();
            output.push(idx as i32);
            t += pt;
        }

        output
    }
}
