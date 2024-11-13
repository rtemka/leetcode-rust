// https://leetcode.com/problems/number-of-1-bits/description/
struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut result = 0;
        for _ in 0..32 {
            result += n & 1;
            n >>= 1;
        }
        result
    }

    pub fn hamming_weight_kernigan(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            n &= n - 1;
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_weight() {
        assert_eq!(3, Solution::hamming_weight(11));
        assert_eq!(30, Solution::hamming_weight(2147483645));
        assert_eq!(1, Solution::hamming_weight(128));

        assert_eq!(30, Solution::hamming_weight_kernigan(2147483645));
    }
}
