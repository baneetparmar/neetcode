impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize;

        fn quick_select(nums: &mut Vec<i32>, l: usize, r: usize, k: usize) -> i32 {
            let pivot = nums[r];
            let mut p = l;
            for i in l..r {
                if nums[i] <= pivot {
                    nums.swap(p, i);
                    p += 1;
                }
            }
            nums.swap(p, r);

            if p > k {
                quick_select(nums, l, p - 1, k)
            } else if p < k {
                quick_select(nums, p + 1, r, k)
            } else {
                nums[p]
            }
        }

        let r = nums.len() - 1;
        quick_select(&mut nums, 0, r, k)
    }
}