// #303
// https://leetcode.com/problems/range-sum-query-immutable/description/
struct NumArray {
    prefix_sum: Vec<i32>,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        for i in 1..nums.len() {
            nums[i] = nums[i] + nums[i - 1];
        }
        Self { prefix_sum: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.prefix_sum[right as usize]
        } else {
            self.prefix_sum[right as usize] - self.prefix_sum[left as usize - 1]
        }
    }
}
