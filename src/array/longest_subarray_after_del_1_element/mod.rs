// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/description
struct Solution;

use std::cmp::max;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut longest: i32 = 0;
        let mut deleted: i8 = 0;

        for hi in 0..nums.len() {
            deleted += (nums[hi] == 0) as i8;

            while deleted > 1 {
                deleted -= (nums[lo] == 0) as i8;
                lo += 1;
            }
            longest = max(longest, (hi - lo) as i32);
        }
        longest
    }

    pub fn longest_subarray3(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, 0);
        let mut longest = 0;
        while hi < nums.len() {
            let mut zeroes = 0;
            while hi < nums.len() && nums[hi] == 0 {
                zeroes += 1;
                hi += 1;
            }
            if hi == nums.len() {
                break;
            }
            let mid = hi;
            println!("lo={};mid={};hi={};longest={}", lo, mid, hi, longest);
            if zeroes > 1 || (lo == 0 && nums[lo] == 0) {
                lo = mid;
            }
            while hi < nums.len() && nums[hi] != 0 {
                hi += 1;
            }
            if lo < mid {
                longest = longest.max(hi - 1 - lo);
                lo = mid;
            } else {
                longest = longest.max(hi - mid);
            }
        }
        println!("lo={};hi={};longest={}", lo, hi, longest);
        longest as i32
        // if lo2 < lo {
        //     longest.max(hi - 1 - lo2) as i32
        // } else {
        //     longest.max(hi - lo) as i32
        // }
    }

    pub fn longest_subarray4(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, 0);
        let mut longest = 0;
        let mut cur = 0;
        let mut mid = 0;
        let mut life = 1;
        while hi < nums.len() {
            cur += nums[hi];
            longest = longest.max(cur);
            match nums[hi] {
                0 if cur == 0 => (),
                0 if life == 0 => {
                    hi = mid;
                    cur = 0;
                }
                0 if life > 0 => {
                    life -= 1;
                    mid = hi;
                }
                _ => (),
            }
            hi += 1;
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_subarray() {
        assert_eq!(3, Solution::longest_subarray(vec![1, 1, 0, 1]));
        // assert_eq!(2, Solution::longest_subarray(vec![1, 1, 1]));
        assert_eq!(
            5,
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1])
        );
    }
}
