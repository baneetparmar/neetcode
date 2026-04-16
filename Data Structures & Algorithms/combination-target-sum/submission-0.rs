impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current = vec![];
        Self::backtrack(&nums, target, 0, &mut current, &mut result);
        result
    }

    fn backtrack(nums: &[i32], remaining: i32, start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            result.push(current.clone());
            return;
        }
        for i in start..nums.len() {
            if nums[i] > remaining { continue; }
            current.push(nums[i]);
            Self::backtrack(nums, remaining - nums[i], i, current, result);
            current.pop();
        }
    }
}