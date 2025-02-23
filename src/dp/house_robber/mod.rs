// https://leetcode.com/problems/house-robber/description
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rob_my(nums: Vec<i32>) -> i32 {
        let mut memo: HashMap<usize, i32> = HashMap::with_capacity(nums.len());
        (Self::rob_rec(&nums, &mut memo, 0)).max(Self::rob_rec(&nums, &mut memo, 1))
    }

    fn rob_rec(nums: &[i32], memo: &mut HashMap<usize, i32>, idx: usize) -> i32 {
        if let Some(&v) = memo.get(&idx) {
            return v;
        }
        if idx >= nums.len() {
            return 0;
        }
        if idx == nums.len() - 1 || (nums.len() > 1 && idx == nums.len() - 2) {
            return nums[idx];
        }
        let res = nums[idx]
            + (Self::rob_rec(nums, memo, idx + 2)).max(Self::rob_rec(nums, memo, idx + 3));
        memo.insert(idx, res);
        res
    }

    // from the solution
    // https://leetcode.com/problems/house-robber/solutions/156523/From-good-to-great.-How-to-approach-most-of-DP-problems/
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut memo = vec![-1; nums.len()];
        Self::rob_rec_memo(&nums, &mut memo, nums.len() - 1)
    }

    fn rob_rec_memo(nums: &[i32], memo: &mut [i32], i: usize) -> i32 {
        if memo[i] >= 0 {
            return memo[i];
        }
        let minus_two = if i > 1 {
            Self::rob_rec_memo(nums, memo, i - 2)
        } else {
            0
        };
        let minus_one = if i > 0 {
            Self::rob_rec_memo(nums, memo, i - 1)
        } else {
            0
        };
        let res = (minus_two + nums[i]).max(minus_one);
        memo[i] = res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn house_robber() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
        assert_eq!(4, Solution::rob(vec![2, 1, 1, 2]));
        assert_eq!(201, Solution::rob(vec![100, 10, 1, 10, 100]));
    }
}
