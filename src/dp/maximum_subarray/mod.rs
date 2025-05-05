// #53
// https://leetcode.com/problems/maximum-subarray/description
// #Kadane's algorithm
// Explanation: https://en.wikipedia.org/wiki/Maximum_subarray_problem
struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut max_ending = nums[0];
        for i in 1..nums.len() {
            max_ending = i32::max(max_ending + nums[i], nums[i]);
            res = i32::max(res, max_ending);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_subarray() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
        assert_eq!(1, Solution::max_sub_array(vec![-2, 1]));
    }
}
