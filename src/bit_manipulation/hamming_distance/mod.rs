// https://leetcode.com/problems/hamming-distance/description/
struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut xor = x ^ y;
        let mut result = 0;
        while xor > 0 {
            xor &= xor - 1; // kernigan approach
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance() {
        assert_eq!(2, Solution::hamming_distance(1, 4));
    }
}
