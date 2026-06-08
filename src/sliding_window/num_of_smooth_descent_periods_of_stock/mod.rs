// #2110
// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/description/

struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        if prices.len() < 2 {
            return 1;
        }

        let mut result = 0;
        let (mut lo, mut hi) = (0, 1);

        while hi <= prices.len() {
            if hi == prices.len() || prices[hi - 1] - prices[hi] != 1 {
                let n = hi - lo;
                result += n * (n + 1) / 2;
                lo = hi;
            }

            hi += 1;
        }

        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_smooth_descent_periods_of_a_stock() {
        assert_eq!(7, Solution::get_descent_periods(vec![3, 2, 1, 4]));

        assert_eq!(11, Solution::get_descent_periods(vec![4, 3, 2, 1, 4]));

        assert_eq!(12, Solution::get_descent_periods(vec![8, 6, 4, 3, 2, 1]));

        assert_eq!(4, Solution::get_descent_periods(vec![8, 6, 7, 7]));
    }
}
