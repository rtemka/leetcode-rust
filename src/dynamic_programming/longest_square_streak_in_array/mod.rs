use std::collections::HashSet;

// https://leetcode.com/problems/longest-square-streak-in-an-array/description
struct Solution;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().cloned().collect();
        let mut result = -1;
        for n in nums {
            // Constraints:
            // 2 <= nums.length <= 10^5
            // 2 <= nums[i] <= 10^5
            let mut squared = n.checked_pow(2).unwrap_or(0);
            let mut running_max = 1;
            while let Some(s) = set.get(&squared) {
                squared = s.checked_pow(2).unwrap_or(0);
                running_max += 1;
                result = result.max(running_max);
            }
        }
        if result < 2 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_square_streak() {
        assert_eq!(3, Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]));
        assert_eq!(-1, Solution::longest_square_streak(vec![2, 3, 5, 6, 7]));
        assert_eq!(2, Solution::longest_square_streak(vec![2, 4]));
        assert_eq!(
            5,
            Solution::longest_square_streak(vec![49, 4, 27, 7, 65536, 16, 6, 13, 256, 21, 2])
        );
        assert_eq!(-1, Solution::longest_square_streak(vec![2]));
    }
}
