// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/
struct Solution;

impl Solution {
    pub fn range_bitwise_and_initial(left: i32, right: i32) -> i32 {
        println!("{}", i32::MAX);
        let lo = f64::from(left).log2().floor() as u32;
        let hi = f64::from(right).log2().floor() as u32;
        let start = if lo < hi { 2_i32.pow(hi) } else { left };
        let mut res = if lo < hi { 0 } else { left };
        println!("lo={};hi={};start={};res={}", lo, hi, start, res);
        println!("left\t{:#034b}", left);
        println!("right\t{:#034b}", right);
        println!("start\t{:#034b}", start);
        for i in start..=right {
            res &= i;
        }
        res
    }

    pub fn range_bitwise_and(left: i32, mut right: i32) -> i32 {
        while right > left {
            right &= right - 1;
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_bitwise_and() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));

        assert_eq!(0, Solution::range_bitwise_and(1, 2147483647));
    }
}
