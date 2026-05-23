// https://leetcode.com/problems/contains-duplicate-ii/description/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (x_idx, &x) in nums.iter().enumerate() {
            match map.insert(x, x_idx) {
                Some(y_idx) if y_idx != x_idx && x_idx.abs_diff(y_idx) <= k => return true,
                _ => (),
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_nearby_duplicate() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
        assert!(Solution::contains_nearby_duplicate(
            vec![0, 1, 2, 3, 4, 0, 0, 7, 8, 9, 10, 11, 12, 0],
            1
        ));
    }
}
