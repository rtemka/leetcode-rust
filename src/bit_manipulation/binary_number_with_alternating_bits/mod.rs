// https://leetcode.com/problems/binary-number-with-alternating-bits/description/
struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev = n & 1 ^ 1;
        while n > 0 {
            if n & 1 == prev {
                return false;
            }
            prev = n & 1;
            n >>= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_alternating_bits() {
        assert!(Solution::has_alternating_bits(5));
        assert!(Solution::has_alternating_bits(10));
        assert!(Solution::has_alternating_bits(21));
        assert!(Solution::has_alternating_bits(85));
        assert!(Solution::has_alternating_bits(5));
        assert!(!Solution::has_alternating_bits(7));
        assert!(!Solution::has_alternating_bits(11));
    }
}
