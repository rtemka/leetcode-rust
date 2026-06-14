// #3689
// https://leetcode.com/problems/maximum-total-subarray-value-i/description

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        (*nums.iter().max().unwrap() as i64 - *nums.iter().min().unwrap() as i64) * k as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_subarray_total_value_1() {
        assert_eq!(4, Solution::max_total_value(vec![1, 2, 3], 2));
    }
}
