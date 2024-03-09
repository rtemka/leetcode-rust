use std::usize;

// https://leetcode.com/problems/search-in-rotated-sorted-array/
struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // println!("\nthe search for the {:?}", nums);
        let pivot = Self::search_pivot_point(&nums);
        // println!("the pivot point is {}", pivot);
        let (mut lo, mut hi) = if target <= nums[nums.len() - 1] {
            (pivot as i32, nums.len() as i32)
        } else {
            (0, pivot as i32)
        };
        while lo <= hi {
            // println!("{:?}:{},{}:mid:{}", nums, lo, hi, ((lo + hi) / 2));
            let mid = (lo + hi) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] > target {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        -1
    }

    pub fn search_better(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = (low + high) / 2;

            if nums[mid as usize] == target {
                return mid;
            }

            if nums[low as usize] <= nums[mid as usize] {
                if nums[low as usize] <= target && target < nums[mid as usize] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[high as usize] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        -1
    }

    pub fn search_pivot_point(nums: &Vec<i32>) -> usize {
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while hi - lo > 1 && nums[lo] > nums[hi] {
            // println!("{:?}:{},{}:mid:{}", nums, lo, hi, ((lo + hi + 1) / 2));
            let mid = (lo + hi + 1) / 2;
            if nums[lo] > nums[mid] {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        if nums[lo] < nums[hi] {
            lo
        } else {
            hi
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_pivot_point() {
        assert_eq!(4, Solution::search_pivot_point(&vec![4, 5, 6, 7, 0, 1, 2]),);
        assert_eq!(1, Solution::search_pivot_point(&vec![3, 1]));
        assert_eq!(0, Solution::search_pivot_point(&vec![1]));
        assert_eq!(0, Solution::search_pivot_point(&vec![1, 3]));
        assert_eq!(0, Solution::search_pivot_point(&vec![1, 2, 3, 4, 5]));
        assert_eq!(
            2,
            Solution::search_pivot_point(&vec![7, 8, 1, 2, 3, 4, 5, 6])
        );
    }

    #[test]
    fn search_in_rotated_sorted_array() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(4, Solution::search(vec![1, 2, 3, 4, 0], 0));
        assert_eq!(0, Solution::search(vec![5, 1, 2, 3, 4], 5));
        assert_eq!(4, Solution::search(vec![0, 1, 2, 3, 4], 4));
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(-1, Solution::search(vec![1], 0));
        assert_eq!(0, Solution::search(vec![1], 1));
        assert_eq!(-1, Solution::search(vec![1], 2));
        assert_eq!(-1, Solution::search(vec![1, 3], 0));
        assert_eq!(-1, Solution::search(vec![1, 3], 2));
        assert_eq!(3, Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2));
    }
}
