impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 { return false; }
        let target = sum / k;
        if nums.iter().any(|&x| x > target) { return false; }

        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a)); // descending, prune early
        let mut buckets = vec![0i32; k as usize];

        fn backtrack(nums: &[i32], buckets: &mut Vec<i32>, idx: usize, target: i32) -> bool {
            if idx == nums.len() { return true; }
            let mut seen = std::collections::HashSet::new();
            for i in 0..buckets.len() {
                if buckets[i] + nums[idx] > target { continue; }
                if !seen.insert(buckets[i]) { continue; } // skip duplicate states
                buckets[i] += nums[idx];
                if backtrack(nums, buckets, idx + 1, target) { return true; }
                buckets[i] -= nums[idx];
            }
            false
        }

        backtrack(&nums, &mut buckets, 0, target)
    }
}