// #239
// https://leetcode.com/problems/sliding-window-maximum/description/
struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len());
        let mut dq: VecDeque<(i32, usize)> = VecDeque::with_capacity(k);
        let (mut lo, mut hi) = (0, 0);
        while hi < nums.len() {
            // Ensure the values of the deque maintain a monotonic decreasing order
            // by removing candidates ≤ the current candidate.
            while let Some(_) = dq.back().filter(|&tuple| tuple.0 <= nums[hi]) {
                dq.pop_back();
            }
            // Add the current candidate.
            dq.push_back((nums[hi], hi));
            // If the window is of length 'k', record the maximum of the window.
            if hi - lo + 1 == k {
                // Remove values whose indexes occur outside the window.
                if dq.front().is_some_and(|&tuple| tuple.1 < lo) {
                    dq.pop_front();
                }
                // The maximum value of this window is the front value in the deque.
                result.push(unsafe { dq.front().unwrap_unchecked().0 });
                // Slide the window by advancing both 'lo' and 'hi'.
                // The `hi` pointer always gets advanced so we just need to advance 'lo'.
                lo += 1;
            }
            hi += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sliding_window_maximum() {
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        )
    }
}
