// #80
// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();
        let (mut lo, mut hi) = (0, 1);

        while hi < n {
            if nums[hi] != nums[lo] {
                if hi - lo > 1 {
                    lo += 2;
                    nums[lo..].rotate_left(hi - lo);
                    n -= hi - lo;
                    hi = lo;
                } else {
                    lo += 1;
                    nums.swap(lo, hi);
                }
            }
            hi += 1;
        }

        nums.truncate(if hi - lo > 1 { lo + 2 } else { hi });
        nums.len() as i32
    }

    pub fn remove_duplicates_better(nums: &mut [i32]) -> i32 {
        let mut writer = 1;
        let mut count = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                count += 1;
            } else {
                count = 1;
            }

            if count <= 2 {
                nums[writer] = nums[i];
                writer += 1;
            }
        }

        writer as i32
    }

    pub fn remove_duplicates_best(nums: &mut [i32]) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut writer = 2;

        for reader in 2..nums.len() {
            if nums[reader] != nums[writer - 2] {
                nums[writer] = nums[reader];
                writer += 1;
            }
        }

        writer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_from_sorted_array_2() {
        let mut arr = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let expected = vec![0, 0, 1, 1, 2, 3, 3];

        Solution::remove_duplicates(&mut arr);

        assert_eq!(expected, arr);

        let mut arr = vec![1, 1, 1, 2, 2, 3];
        let expected = vec![1, 1, 2, 2, 3];

        Solution::remove_duplicates(&mut arr);

        assert_eq!(expected, arr);

        let mut arr = vec![1, 1, 1];
        let expected = vec![1, 1];

        Solution::remove_duplicates(&mut arr);

        assert_eq!(expected, arr);
    }
}
