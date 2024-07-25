// https://leetcode.com/problems/find-peak-element/description
struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] > nums[mid + 1] && (mid == 0 || nums[mid] > nums[mid - 1]) {
                return mid as i32;
            }
            if nums[mid] < nums[mid + 1] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_peak_element() {
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
        assert_eq!(
            1,
            Solution::find_peak_element(vec![10, 12, 10, 9, 8, 7, 6, 5, 7, 8])
        );
        assert_eq!(0, Solution::find_peak_element(vec![1]));
        assert_eq!(0, Solution::find_peak_element(vec![2, 1]));
    }
}
