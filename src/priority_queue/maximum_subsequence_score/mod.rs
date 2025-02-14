// https://leetcode.com/problems/maximum-subsequence-score/description/
struct Solution;

use std::{cmp, collections::BinaryHeap};

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut pairs: Vec<(i32, i32)> = nums2.into_iter().zip(nums1.into_iter()).collect();
        pairs.sort_unstable();
        let mut max = 0;
        let mut sum = 0;
        let mut heap: BinaryHeap<cmp::Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
        for (n2, n1) in pairs.into_iter().rev() {
            sum += n1 as i64;
            heap.push(cmp::Reverse(n1));
            if heap.len() > k {
                sum -= heap.pop().unwrap_or(cmp::Reverse(0)).0 as i64;
            }
            if heap.len() == k {
                max = max.max(sum * n2 as i64);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_subsequnce_score() {
        assert_eq!(
            12,
            Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3)
        );
    }
}
