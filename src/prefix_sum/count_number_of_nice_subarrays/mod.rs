// #1248
// https://leetcode.com/problems/count-number-of-nice-subarrays/description/
//
// ╔══════════════════════════════════════════════════════════════════════════════════════════════╗
// ║  TASK: Count the number of subarrays containing exactly `k` odd numbers ("nice" subarrays).  ║
// ║                                                                                              ║
// ║  KEY INSIGHT: Replace every number with 0 (even) or 1 (odd).                                 ║
// ║               Now the task becomes: "Count subarrays with sum exactly k".                    ║
// ║               This is the classic "Subarray Sum Equals K" problem.                           ║
// ╚══════════════════════════════════════════════════════════════════════════════════════════════╝
//
// ──────────────────────────────────────────────────────────────────────────────────────────────
// EXAMPLE WALKTHROUGH: nums = [1, 1, 2, 1, 1],  k = 3
// ──────────────────────────────────────────────────────────────────────────────────────────────
//
//   Step 0:  Initialise hash map with { 0 → 1 }
//            (one prefix with sum=0 exists before we start — the empty prefix)
//
//   Index :     0      1      2      3      4
//   nums  :    [1]    [1]    [2]    [1]    [1]
//   odd?  :     1      1      0      1      1
//                ↓      ↓      ↓      ↓      ↓
//   prefix_sum:  1      2      2      3      4
//
//   We are looking for previous prefix sums equal to (current_sum - k).
//   Every time we find such a sum, it means the elements BETWEEN those two
//   prefixes contain exactly k odd numbers.
//
//   ┌────────────────────────────────────────────────────────────────────────┐
//   │  i=0, val=1  │  odd? yes → sum=1                                      │
//   │              │  need sum-k = 1-3 = -2   → map[-2] = 0                 │
//   │              │  result += 0                                             │
//   │              │  map { 0:1, 1:1 }                                      │
//   ├────────────────────────────────────────────────────────────────────────┤
//   │  i=1, val=1  │  odd? yes → sum=2                                      │
//   │              │  need sum-k = 2-3 = -1   → map[-1] = 0                 │
//   │              │  result += 0                                             │
//   │              │  map { 0:1, 1:1, 2:1 }                                 │
//   ├────────────────────────────────────────────────────────────────────────┤
//   │  i=2, val=2  │  odd? no  → sum=2   (even adds nothing)                │
//   │              │  need sum-k = 2-3 = -1   → map[-1] = 0                 │
//   │              │  result += 0                                             │
//   │              │  map { 0:1, 1:1, 2:2 }                                 │
//   ├────────────────────────────────────────────────────────────────────────┤
//   │  i=3, val=1  │  odd? yes → sum=3   🎯 TARGET SUM REACHED!            │
//   │              │  need sum-k = 3-3 = 0    → map[0] = 1  ← FOUND ONE!   │
//   │              │  result += 1                                             │
//   │              │  map { 0:1, 1:1, 2:2, 3:1 }                            │
//   │              │                                                          │
//   │              │  Visualization:                                        │
//   │              │  prefix 0 covers []                                    │
//   │              │  prefix 3 covers [1, 1, 2, 1]  → 3 odds inside ✓       │
//   ├────────────────────────────────────────────────────────────────────────┤
//   │  i=4, val=1  │  odd? yes → sum=4                                      │
//   │              │  need sum-k = 4-3 = 1    → map[1] = 1  ← FOUND ONE!   │
//   │              │  result += 1                                             │
//   │              │  map { 0:1, 1:1, 2:2, 3:1, 4:1 }                       │
//   │              │                                                          │
//   │              │  Visualization:                                        │
//   │              │  prefix 1 covers [1]                                   │
//   │              │  prefix 4 covers [1, 1, 2, 1, 1]                       │
//   │              │  difference:      [1, 2, 1, 1]  → 3 odds inside ✓      │
//   └────────────────────────────────────────────────────────────────────────┘
//
//   FINAL RESULT = 2   (subarrays [0..3] and [1..4])
//
// ──────────────────────────────────────────────────────────────────────────────────────────────
// WHY IT WORKS:
// ──────────────────────────────────────────────────────────────────────────────────────────────
//
//   If prefix_sum[j] = X  and  prefix_sum[i] = X + k   (where j < i)
//
//        0         j         i
//        │─────────│─────────│
//        └─ sum=X ─┘         │
//        └──── sum=X+k ──────┘
//                 ↑_________↑
//                    subarray(j+1 .. i) has sum = k
//
//   Counting how many times each prefix sum occurred lets us find matching
//   pairs in O(1) per element, giving overall O(n) time.
// ──────────────────────────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        // `sum` = number of odd elements seen so far (a.k.a. prefix sum of 0/1 array)
        let mut sum = 0;

        // Maps each prefix_sum value → how many times it has occurred.
        // We pre-fill with {0: 1} because before the array starts we have
        // one empty prefix whose sum is 0. This handles the case when a
        // subarray starting from index 0 itself has exactly k odds.
        let mut prefix_sum: HashMap<i32, i32> = HashMap::with_capacity(nums.len() + 1);
        prefix_sum.insert(0, 1);

        for val in nums {
            // Convert current number: odd → 1, even → 0, and add to running prefix sum.
            sum += (val % 2 > 0) as i32;

            // If some earlier prefix had sum = (current_sum - k),
            // then the elements between that earlier point and now
            // contain exactly k odd numbers.
            result += prefix_sum.get(&(sum - k)).unwrap_or(&0);

            // Record the current prefix sum so future elements can pair with it.
            prefix_sum
                .entry(sum)
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
    fn count_number_of_nice_subarrays() {
        assert_eq!(2, Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3));

        assert_eq!(0, Solution::number_of_subarrays(vec![2, 4, 6], 1));

        assert_eq!(
            16,
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)
        );
    }
}
