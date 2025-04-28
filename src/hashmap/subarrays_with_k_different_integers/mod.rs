// #992
// https://leetcode.com/problems/subarrays-with-k-different-integers/description/
// Explanation: https://leetcode.com/problems/subarrays-with-k-different-integers/editorial/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_with_at_most_distinct(&nums, k as usize)
            - Self::subarrays_with_at_most_distinct(&nums, k as usize - 1)
    }

    pub fn subarrays_with_at_most_distinct(nums: &[i32], k: usize) -> i32 {
        let mut map = HashMap::new();
        let mut count = 0;
        let mut lo = 0;

        for hi in 0..nums.len() {
            map.entry(nums[hi])
                .and_modify(|count| *count += 1)
                .or_insert(1);
            while map.len() > k {
                if let Some(val) = map.get_mut(&nums[lo]) {
                    *val -= 1;
                    if *val == 0 {
                        map.remove(&nums[lo]);
                    }
                }
                lo += 1;
            }
            count += hi - lo + 1;
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarrays_with_k_different_integers() {
        assert_eq!(
            7,
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2)
        );

        assert_eq!(
            3,
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3)
        );
    }
}
