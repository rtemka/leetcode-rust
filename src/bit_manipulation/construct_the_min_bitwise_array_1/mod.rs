// https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/description/
struct Solution;

impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        while i < nums.len() {
            nums[i] = Self::min_or_plus_one(nums[i]);
            i += 1;
        }
        nums
    }

    #[inline]
    fn min_or_plus_one(n: i32) -> i32 {
        for i in 0..n {
            if (i | (i + 1)) == n {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_bitwise_array() {
        assert_eq!(
            vec![-1, 1, 4, 3],
            Solution::min_bitwise_array(vec![2, 3, 5, 7])
        );
        assert_eq!(
            vec![9, 12, 15],
            Solution::min_bitwise_array(vec![11, 13, 31])
        );
    }
}
