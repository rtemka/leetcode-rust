// #55
// https://leetcode.com/problems/jump-game/dscription
// See Also: #45
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        Self::can_jump_greedy(nums)
    }

    fn can_jump_greedy(nums: Vec<i32>) -> bool {
        // Set the initial destination to the last index in the array.
        let mut destination = nums.len() - 1;
        // Traverse the array in reverse to see if the destination can be reached by earlier indexes.
        for (i, n) in nums.into_iter().enumerate().rev() {
            // If we can reach the destination from the current index, set this index as the new destination.
            if i + n as usize >= destination {
                destination = i;
            }
        }
        // If the destination is index 0, we can jump to the end from index 0.
        destination == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_game() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
