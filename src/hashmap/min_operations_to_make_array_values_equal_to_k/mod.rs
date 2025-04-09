// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/description
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for n in nums.into_iter().collect::<HashSet<i32>>() {
            if n < k {
                return -1;
            }
            if n > k {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_operations_to_make_array_values_equal_to_k() {
        assert_eq!(2, Solution::min_operations(vec![5, 2, 5, 4, 5], 2));
        assert_eq!(-1, Solution::min_operations(vec![2, 1, 2], 2));
        assert_eq!(4, Solution::min_operations(vec![9, 7, 5, 3], 1));
    }
}
