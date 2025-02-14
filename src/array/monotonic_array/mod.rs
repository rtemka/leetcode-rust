// https://leetcode.com/problems/monotonic-array/description/
struct Solution;

use std::cmp;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        Self::is_monotonic_helper(&nums, cmp::Ordering::Greater)
            || Self::is_monotonic_helper(&nums, cmp::Ordering::Less)
    }

    fn is_monotonic_helper(nums: &[i32], wrong_ordering: cmp::Ordering) -> bool {
        for i in 1..nums.len() {
            if nums[i - 1].cmp(&nums[i]) == wrong_ordering {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotonic_array() {
        assert!(Solution::is_monotonic(vec![1]));
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }
}
