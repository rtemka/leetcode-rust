// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut res = 0;
        for p in prices.into_iter().rev() {
            if p > max {
                max = p;
            } else {
                res = res.max(max - p);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_time_to_buy_and_sell_stock() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
        assert_eq!(2, Solution::max_profit(vec![2, 4, 1]));
        assert_eq!(4, Solution::max_profit(vec![3, 2, 6, 5, 0, 3]));
        assert_eq!(6, Solution::max_profit(vec![6, 1, 3, 2, 4, 7]));
        assert_eq!(0, Solution::max_profit(vec![10000, 10000, 10000, 10000]));
        assert_eq!(0, Solution::max_profit(vec![1]));
    }
}
