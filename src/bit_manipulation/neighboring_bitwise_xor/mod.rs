// https://leetcode.com/problems/neighboring-bitwise-xor/description
struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().fold(0, |acc, x| acc ^ x) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighboring_bitwise_xor() {
        assert!(Solution::does_valid_array_exist(vec![1, 1, 0]));
        assert!(Solution::does_valid_array_exist(vec![1, 1]));
        assert!(!Solution::does_valid_array_exist(vec![1, 0]));
    }
}
