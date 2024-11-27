// https://leetcode.com/problems/binary-gap/description/
struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        // println!("{}", 0b1000000001);
        // println!("{:#034b}", n);
        //        Constraints:
        // 1 <= n <= 10^9
        let mut max_gap = 0;
        let (mut lo, mut hi) = (32, 0);
        while n > 0 {
            if n & 1 == 0 {
                hi += 1;
            } else {
                max_gap = max_gap.max(hi - lo);
                (lo, hi) = (hi, hi + 1);
            }
            n >>= 1;
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_gap() {
        assert_eq!(1, Solution::binary_gap(6));
        assert_eq!(9, Solution::binary_gap(513));
        assert_eq!(2, Solution::binary_gap(22));
        assert_eq!(0, Solution::binary_gap(8));
        assert_eq!(2, Solution::binary_gap(5));
        assert_eq!(1, Solution::binary_gap(1023));
    }
}
