// https://leetcode.com/problems/count-complete-subarrays-in-an-array/description
struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let disctinct = nums.iter().cloned().collect::<HashSet<i32>>().len();

        let mut map = HashMap::new();
        let (mut lo, mut hi) = (0, 0);
        let mut count = 0;
        while hi < nums.len() {
            map.entry(nums[hi])
                .and_modify(|count| *count += 1)
                .or_insert(1);

            while map.len() == disctinct {
                count += nums.len() - hi;
                let val = unsafe { map.get_mut(&nums[lo]).unwrap_unchecked() };
                *val -= 1;
                if *val == 0 {
                    map.remove(&nums[lo]);
                }
                lo += 1;
            }

            hi += 1;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete_subarrays_in_array() {
        assert_eq!(4, Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]));
        assert_eq!(10, Solution::count_complete_subarrays(vec![5, 5, 5, 5]));
    }
}
