// #45
// https://leetcode.com/problems/jump-game-ii/description
struct Solution;

impl Solution {
    // My initial DP approach (not very efficient).
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut memo = vec![i32::MAX; nums.len()];
        memo[nums.len() - 1] = 0;
        Self::jump_rec(&nums, &mut memo, 0)
    }

    fn jump_rec(nums: &[i32], memo: &mut [i32], i: usize) -> i32 {
        if memo[i] < i32::MAX || nums[i] == 0 {
            return memo[i];
        }
        if nums[i] as usize >= nums.len() - 1 - i {
            memo[i] = 1;
            return memo[i];
        }
        let max_jump = i + nums[i] as usize + 1;
        for k in i + 1..max_jump {
            memo[i] = i32::min(memo[i], Self::jump_rec(nums, memo, k));
        }
        memo[i] = memo[i].saturating_add(1); // add jump from this one.
        memo[i]
    }

    // The efficient solution.
    pub fn jump2(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;

        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);

            if i == current_end {
                jumps += 1;
                current_end = farthest;
            }
        }

        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_game_2() {
        assert_eq!(3, Solution::jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]));
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
        assert_eq!(3, Solution::jump(vec![2, 1, 2, 3, 2, 1, 4]));
        assert_eq!(0, Solution::jump(vec![0]));
        assert_eq!(3, Solution::jump(vec![2, 1, 1, 1, 1]));
        assert_eq!(2, Solution::jump(vec![4, 1, 1, 3, 1, 1, 1]));
    }
}
