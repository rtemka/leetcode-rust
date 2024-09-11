// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description
struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_bit_flips() {
        assert_eq!(3, Solution::min_bit_flips(10, 7));
        assert_eq!(3, Solution::min_bit_flips(3, 4));
    }
}
