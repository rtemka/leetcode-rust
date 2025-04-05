// https://leetcode.com/problems/sum-of-all-subset-xor-totals/description
struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 1..=nums.len() {
            Self::backtrack(&nums, &mut sum, 0, 0, i);
        }
        sum
    }

    fn backtrack(nums: &[i32], sum: &mut i32, cur_sum: i32, cur: usize, max: usize) {
        // if nums.len() < max - cur {
        //     return;
        // }
        if cur == max {
            *sum += cur_sum;
            return;
        }
        for (i, &n) in nums.iter().enumerate() {
            Self::backtrack(&nums[i + 1..], sum, cur_sum ^ n, cur + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_all_subset_xor_totals() {
        assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
        assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
        assert_eq!(480, Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]));
    }
}
