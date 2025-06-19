// #34
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Self::find_by_bin_search_and_pp(&nums, target)
        Self::find_by_bin_search(&nums, target)
    }

    fn find_by_bin_search_and_pp(nums: &[i32], target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(_) => {
                vec![
                    nums.partition_point(|&x| x < target) as i32,
                    nums.partition_point(|&x| x <= target) as i32 - 1,
                ]
            }
            Err(_) => vec![-1, -1],
        }
    }

    #[inline]
    fn find_by_bin_search(nums: &[i32], target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(_) => {
                vec![
                    Self::binary_search_leftmost_index(nums, target) as i32,
                    Self::binary_search_leftmost_index(nums, target + 1) as i32 - 1,
                ]
            }
            Err(_) => vec![-1, -1],
        }
    }

    #[inline]
    fn binary_search_leftmost_index(nums: &[i32], target: i32) -> usize {
        let n = nums.len();
        let (mut lo, mut hi) = (0, n);
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first_and_last_position_of_elem_in_sorted_array() {
        assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }
}
