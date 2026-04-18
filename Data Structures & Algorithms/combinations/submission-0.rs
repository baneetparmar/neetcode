impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(start:i32, end:i32,k:i32,cur:&mut Vec<i32>, res:&mut Vec<Vec<i32>>) {
            if cur.len() == k as usize {
                res.push(cur.clone());
                return;
            }

            for i in start..=end {
                cur.push(i);
                backtrack(i + 1, end,k, cur, res);
                cur.pop();
            }
        }

        let mut result = vec![];
        let mut current = vec![];
        backtrack(1, n, k , &mut current, &mut result);
        result
    }
}
