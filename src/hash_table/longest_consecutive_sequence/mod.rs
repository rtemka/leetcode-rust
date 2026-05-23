// #128
// https://leetcode.com/problems/longest-consecutive-sequence/description/
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut longest_seq = 0;
        for &n in set.iter() {
            // If the current number is the smallest number in its sequence,
            // search for the length of its chain.
            if !set.contains(&(n - 1)) {
                let mut n = n;
                let mut current_seq = 1;
                // Continue to find the next consecutive numbers in the sequence.
                while set.contains(&(n + 1)) {
                    n += 1;
                    current_seq += 1;
                }
                // Update the maximum sequence so far.
                longest_seq = i32::max(longest_seq, current_seq);
            }
        }
        longest_seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_consecutive_sequence() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }
}
