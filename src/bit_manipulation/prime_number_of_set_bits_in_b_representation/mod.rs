// https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation/description/
struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right).fold(0, |acc, n| match n.count_ones() {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 => acc + 1,
            _ => acc,
        })
    }

    #[inline]
    fn count_bits(mut n: i32) -> usize {
        let mut count = 0;
        while n > 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_prime_set_bits() {
        assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    }
}
