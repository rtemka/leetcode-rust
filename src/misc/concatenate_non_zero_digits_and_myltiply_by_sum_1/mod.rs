// #3754
// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i
// See also: #3756

struct Solution;

impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut sum = 0;
        let mut x = 0;
        let mut power10 = 1;

        while n > 0 {
            let digit = n % 10;
            if digit > 0 {
                x += digit * power10;
                sum += digit;
                power10 *= 10;
            }
            n /= 10;
        }

        x as i64 * sum as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenate_non_zero_digits_and_multiply_by_sum_i() {
        assert_eq!(12340, Solution::sum_and_multiply(10203004));
    }
}
