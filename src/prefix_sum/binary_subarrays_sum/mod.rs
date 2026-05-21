// #930
// https://leetcode.com/problems/binary-subarrays-with-sum/description/
//
// Problem: Count the number of subarrays with sum equal to `goal`.
// Array contains only 0s and 1s.
//
// ┌─────────────────────────────────────────────────────────────────────────────┐
// │  INTUITION: Prefix Sum + HashMap                                            │
// │                                                                             │
// │  For any subarray nums[i..=j], its sum = prefix[j+1] - prefix[i]           │
// │  We want: prefix[j+1] - prefix[i] = goal                                   │
// │  Therefore: prefix[i] = prefix[j+1] - goal                                 │
// │                                                                             │
// │  As we iterate, we keep track of how many times each prefix sum occurred.  │
// │  At position j, the number of valid subarrays ending here equals           │
// │  the count of prefix sums equal to (current_prefix - goal).                │
// └─────────────────────────────────────────────────────────────────────────────┘

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut result = 0;
        let mut running_sum = 0;

        // prefix_sum maps:  prefix_value → how many times we've seen it
        // Example: prefix_sum[3] = 2 means the prefix sum 3 occurred twice
        let mut prefix_sum: HashMap<i32, i32> = HashMap::with_capacity(nums.len() + 1);

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  WHY insert(0, 1)?                                                  │
        // │                                                                     │
        // │  Imagine the subarray starts from index 0.                         │
        // │  Its sum = prefix[j+1] - prefix[0] = prefix[j+1] - 0               │
        // │  We need prefix[0] = 0 to exist in the map for the formula to work.│
        // │  This represents the "empty prefix" before the array begins.       │
        // └─────────────────────────────────────────────────────────────────────┘
        prefix_sum.insert(0, 1);

        for val in nums {
            // Expand the running prefix sum
            running_sum += val;

            // ┌─────────────────────────────────────────────────────────────────┐
            // │  VISUALIZATION for nums = [1, 0, 1, 0, 1], goal = 2             │
            // │                                                                 │
            // │  index:      0   1   2   3   4                                  │
            // │  nums:      [1] [0] [1] [0] [1]                                 │
            // │                                                                 │
            // │  prefix:   0   1   1   2   2   3                                 │
            // │            ↑                                                    │
            // │         initial (empty subarray)                                 │
            // │                                                                 │
            // │  Step 0: running_sum = 0, map = {0: 1}                          │
            // │                                                                 │
            // │  Step 1: val = 1, running_sum = 1                               │
            // │          need prefix = 1 - 2 = -1  → not in map                  │
            // │          result += 0                                              │
            // │          map = {0: 1, 1: 1}                                       │
            // │                                                                 │
            // │  Step 2: val = 0, running_sum = 1                               │
            // │          need prefix = 1 - 2 = -1  → not in map                  │
            // │          result += 0                                              │
            // │          map = {0: 1, 1: 2}                                       │
            // │                                                                 │
            // │  Step 3: val = 1, running_sum = 2                               │
            // │          need prefix = 2 - 2 = 0   → map[0] = 1  ✓               │
            // │          result += 1   (subarray [0..2] = [1,0,1])               │
            // │          map = {0: 1, 1: 2, 2: 1}                                 │
            // │                                                                 │
            // │  Step 4: val = 0, running_sum = 2                               │
            // │          need prefix = 2 - 2 = 0   → map[0] = 1  ✓               │
            // │          result += 1   (subarray [0..3] = [1,0,1,0])             │
            // │          map = {0: 1, 1: 2, 2: 2}                                 │
            // │                                                                 │
            // │  Step 5: val = 1, running_sum = 3                               │
            // │          need prefix = 3 - 2 = 1   → map[1] = 2  ✓✓              │
            // │          result += 2   (subarrays [1..4] = [0,1,0,1],           │
            // │                         and [2..4] = [1,0,1])                    │
            // │          map = {0: 1, 1: 2, 2: 2, 3: 1}                           │
            // │                                                                 │
            // │  Final result = 4                                               │
            // └─────────────────────────────────────────────────────────────────┘
            result += prefix_sum.get(&(running_sum - goal)).unwrap_or(&0);

            // Record the current prefix sum for future positions
            prefix_sum
                .entry(running_sum)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_subarrays_sum() {
        // Example from LeetCode:
        // nums = [1,0,1,0,1], goal = 2
        // Subarrays with sum 2:
        //   [1,0,1]      (indices 0..2)
        //   [1,0,1,0]    (indices 0..3)
        //   [0,1,0,1]    (indices 1..4)
        //   [1,0,1]      (indices 2..4)
        assert_eq!(4, Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));

        // Edge case: all zeros, goal = 0
        // Every possible subarray sums to 0.
        // For n=5: 5 + 4 + 3 + 2 + 1 = 15 subarrays
        assert_eq!(15, Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0));
    }
}
