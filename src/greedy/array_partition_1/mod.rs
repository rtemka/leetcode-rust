// https://leetcode.com/problems/array-partition/
struct Solution {}

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
        // let mut i = 0;
        // let mut sum = 0;
        // while i < nums.len() {
        //     sum += nums[i];
        //     i += 2;
        // }
        // sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_pair_sum() {
        assert_eq!(4, Solution::array_pair_sum(vec![1, 4, 3, 2]));
        assert_eq!(9, Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]));
    }
}
