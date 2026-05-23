struct Solution {}

// https://leetcode.com/problems/sort-array-by-parity/description/
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo < hi {
            if nums[lo] & 1 == 0 {
                lo += 1;
                continue;
            }
            nums.swap(lo, hi);
            hi -= 1;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array_by_parity() {
        assert_eq!(
            vec![4, 2, 1, 3],
            Solution::sort_array_by_parity(vec![3, 1, 2, 4])
        );

        assert_eq!(
            vec![4, 4, 8, 2, 6, 7, 5, 3, 1],
            Solution::sort_array_by_parity(vec![1, 3, 5, 7, 6, 2, 8, 4, 4])
        );
        assert_eq!(vec![0], Solution::sort_array_by_parity(vec![0]));
    }
}
