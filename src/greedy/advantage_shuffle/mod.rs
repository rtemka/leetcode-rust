// #870
// https://leetcode.com/problems/advantage-shuffle/description/

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn advantage_count(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();

        let mut nums1: VecDeque<i32> = nums1.into();
        let mut nums2: Vec<(usize, i32)> = nums2.into_iter().enumerate().collect();

        nums2.sort_unstable_by_key(|k| std::cmp::Reverse(k.1));

        let mut result = vec![0; nums2.len()];

        for (i, n) in nums2 {
            if let Some(&max_from_nums1) = nums1.back()
                && max_from_nums1 > n
            {
                result[i] = nums1.pop_back().unwrap();
            } else {
                result[i] = nums1.pop_front().unwrap();
            }
        }

        result
    }
}
