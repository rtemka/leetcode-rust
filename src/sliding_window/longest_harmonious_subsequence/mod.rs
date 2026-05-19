// #594
// https://leetcode.com/problems/longest-harmonious-subsequence/description/

struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let (mut lo, mut hi) = (0, 0);
        let mut longest = 0;

        while hi < nums.len() {
            let diff = nums[hi] - nums[lo];

            if diff > 1 {
                lo += 1;
                continue;
            }

            if diff == 1 {
                longest = longest.max(hi - lo + 1);
            }

            hi += 1;
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_harmonious_subsequence() {
        assert_eq!(5, Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));

        assert_eq!(2, Solution::find_lhs(vec![1, 2, 3, 4]));

        assert_eq!(0, Solution::find_lhs(vec![1, 1, 1, 1]));

        assert_eq!(4, Solution::find_lhs(vec![1, 2, 2, 1]));
    }
}
