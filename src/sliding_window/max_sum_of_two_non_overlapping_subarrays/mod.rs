// #1031
// https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/description/

struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(_nums: Vec<i32>, _first_len: i32, _second_len: i32) -> i32 {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sum_of_two_non_overlapping_subarrays() {
        assert_eq!(
            20,
            Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2)
        );
    }
}
