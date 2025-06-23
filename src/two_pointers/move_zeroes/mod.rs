// #283
// https://leetcode.com/problems/move-zeroes/description
struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut lo = 0;
        for hi in 0..nums.len() {
            if nums[hi] != 0 {
                nums.swap(lo, hi);
                lo += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_zeroes() {
        let mut arr = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut arr);
        assert_eq!(vec![1, 3, 12, 0, 0], arr);
    }
}
