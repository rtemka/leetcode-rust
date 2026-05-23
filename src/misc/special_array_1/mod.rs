// https://leetcode.com/problems/special-array-i/description/
struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut prev = nums[0] % 2;
        for i in 1..nums.len() {
            if nums[i] % 2 == prev {
                return false;
            }
            prev = nums[i] % 2;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_array_special_1() {
        assert!(!Solution::is_array_special(vec![4, 3, 1, 6]));
        assert!(Solution::is_array_special(vec![2, 1, 4]));
    }
}
