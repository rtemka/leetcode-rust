use std::usize;

// https://leetcode.com/problems/rotate-array/description/
struct Solution {}

impl Solution {
    // O(1) space; O(n*2) time -> Time limit exceeded on leetcode.
    pub fn rotate_slow(nums: &mut [i32], k: i32) {
        for _ in 0..k {
            let mut i = nums.len() - 1;
            while i != 0 {
                nums.swap(i, i - 1);
                i -= 1;
            }
        }
    }

    // O(n) space; O(n) time
    #[allow(clippy::all)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize;
        let k = k % nums.len();
        let r = nums.len() - k;
        *nums = [&nums[r..], &nums[..r]].concat();
    }

    pub fn rotate_cheat(nums: &mut [i32], k: i32) {
        let k = k as usize;
        let k = k % nums.len();
        nums.rotate_right(k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_array() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate_slow(&mut arr, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], arr,);

        let mut arr = vec![-1, -100, 3, 99];
        Solution::rotate_slow(&mut arr, 2);
        assert_eq!(vec![3, 99, -1, -100], arr,);

        let mut arr = vec![1];
        Solution::rotate_slow(&mut arr, 2);
        assert_eq!(vec![1], arr,);

        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut arr, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], arr,);

        let mut arr = vec![-1, -100, 3, 99];
        Solution::rotate(&mut arr, 2);
        assert_eq!(vec![3, 99, -1, -100], arr,);

        let mut arr = vec![1];
        Solution::rotate(&mut arr, 1);
        assert_eq!(vec![1], arr,);

        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate_cheat(&mut arr, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], arr,);

        let mut arr = vec![-1, -100, 3, 99];
        Solution::rotate_cheat(&mut arr, 2);
        assert_eq!(vec![3, 99, -1, -100], arr,);

        let mut arr = vec![1];
        Solution::rotate_cheat(&mut arr, 2);
        assert_eq!(vec![1], arr,);
    }
}
