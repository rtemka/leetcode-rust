use std::collections::BinaryHeap;

// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
struct Solution;

impl Solution {
    // cheat whit sorting
    pub fn find_kth_largest_sorting(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        println!("{:#?}", nums);
        nums[k as usize - 1]
    }

    // binary heap from std
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut n = 0;
        for _ in 0..k {
            // safety: k-th element MUST be in heap according to the problem description
            n = unsafe { heap.pop().unwrap_unchecked() };
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_kth_largest() {
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
        assert_eq!(
            4,
            Solution::find_kth_largest_sorting(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
        assert_eq!(
            5,
            Solution::find_kth_largest_sorting(vec![3, 2, 1, 5, 6, 4], 2)
        );
    }
}
