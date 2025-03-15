// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/description
struct Solution;

impl Solution {
    // not attempted, possibly wrong
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        if nums[0] > 0 || nums[nums.len() - 1] < 0 {
            return nums.len() as i32;
        }
        let mut i = 0;
        let mut z = 0;
        while nums[i] < 1 && i < nums.len() {
            i += 1;
            z += (nums[i] == 0) as usize;
        }
        let y = nums.len().abs_diff(i);
        y.max(i - z) as i32
    }

    pub fn maximum_count_bin_search(nums: Vec<i32>) -> i32 {
        match nums.binary_search(&0) {
            Ok(i) => {
                let (mut lo, mut hi) = (i, i);
                while lo > 0 && nums[lo] == 0 {
                    lo -= 1;
                }
                while hi < nums.len() && nums[hi] == 0 {
                    hi += 1;
                }
                (nums.len() - hi).max(if nums[lo] == 0 { lo } else { lo + 1 }) as i32
            }
            Err(i) => i.max(nums.len() - i) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_count_of_pos_and_neg_integer() {
        assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
        assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
        assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
        assert_eq!(
            3,
            Solution::maximum_count_bin_search(vec![-3, -2, -1, 0, 0, 1, 2])
        );
        assert_eq!(
            3,
            Solution::maximum_count_bin_search(vec![-2, -1, -1, 1, 2, 3])
        );
        assert_eq!(4, Solution::maximum_count_bin_search(vec![5, 20, 66, 1314]));

        assert_eq!(0, Solution::maximum_count_bin_search(vec![0, 0, 0, 0]));
        assert_eq!(
            3,
            Solution::maximum_count_bin_search(vec![0, 0, 0, 0, 0, 1, 2, 3])
        );

        assert_eq!(
            3,
            Solution::maximum_count_bin_search(vec![-3, -2, -1, 0, 0, 0, 0])
        );
        assert_eq!(1, Solution::maximum_count_bin_search(vec![-1, 0, 0, 0, 0]));
        assert_eq!(1, Solution::maximum_count_bin_search(vec![-200]));
        assert_eq!(1, Solution::maximum_count_bin_search(vec![200]));
        assert_eq!(0, Solution::maximum_count_bin_search(vec![0]));
        assert_eq!(1, Solution::maximum_count_bin_search(vec![0, 0, 0, 0, 1]));
        assert_eq!(2, Solution::maximum_count_bin_search(vec![-1, 1, 1]));
        assert_eq!(
            7,
            Solution::maximum_count_bin_search(vec![
                -1764, -1562, -1226, -1216, -402, -386, -133, 979, 1227, 1992
            ])
        );
    }
}
