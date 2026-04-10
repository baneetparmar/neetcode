impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for &num in &nums {
            sum |= num;
        }
        sum << (nums.len() - 1)
    }
}