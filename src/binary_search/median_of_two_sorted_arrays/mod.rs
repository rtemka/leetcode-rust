// #4
// https://leetcode.com/problems/median-of-two-sorted-arrays/description/
struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Optimization: ensure 'nums1' is the smaller array.
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let (m, n) = (nums1.len(), nums2.len());
        let (mut left, mut right) = (0, m);
        // A median always exists in a non-empty array, so continue binary search until it’s found.
        loop {
            let l1_idx = (left + right) / 2;
            let l2_idx = (m + n + 1) / 2 - l1_idx;
            // Set to MIN or MAX if out of bounds.
            let l1 = if l1_idx == 0 {
                i32::MIN
            } else {
                nums1[l1_idx as usize - 1]
            };
            let r1 = if l1_idx == m {
                i32::MAX
            } else {
                nums1[(l1_idx) as usize]
            };
            let l2 = if l2_idx == 0 {
                i32::MIN
            } else {
                nums2[l2_idx as usize - 1]
            };
            let r2 = if l2_idx == n {
                i32::MAX
            } else {
                nums2[(l2_idx) as usize]
            };
            // If 'L1 > R2', then 'L1' is too far to the right. Narrow the search space toward the left.
            if l1 > r2 {
                right = l1_idx - 1;
            // If 'L2 > R1', then 'L1' is too far to the left. Narrow the search space toward the right.
            } else if l2 > r1 {
                left = l1_idx + 1;
            // If both 'L1' and 'L2' are less than or equal to both 'R1' and 'R2', we found the correct slice.
            } else {
                if (m + n) % 2 == 0 {
                    let max_left = i32::max(l1, l2);
                    let min_right = i32::min(r1, r2);
                    return (max_left + min_right) as f64 / 2.0;
                } else {
                    return i32::max(l1, l2) as f64;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_of_two_sorted_arrays() {
        assert_eq!(
            4.0,
            Solution::find_median_sorted_arrays(vec![0, 2, 5, 6, 8], vec![1, 3, 7])
        );
        assert_eq!(
            5.0,
            Solution::find_median_sorted_arrays(vec![0, 2, 5, 6, 8], vec![1, 3, 7, 9])
        );
        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
    }
}
