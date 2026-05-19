// #961
// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for n in nums.into_iter() {
            if !set.insert(n) {
                return n;
            }
        }
        unreachable!();
    }

    pub fn repeated_n_times_better(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i + 2 <= nums.len() {
            if nums[i] == nums[i + 1] || nums[i] == nums[i + 2] {
                return nums[i];
            }
            i = i + 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_repeated_element_in_size_2n_array() {
        assert_eq!(3, Solution::repeated_n_times_better(vec![1, 2, 3, 3]));
        assert_eq!(2, Solution::repeated_n_times_better(vec![2, 1, 2, 5, 3, 2]));
    }
}
