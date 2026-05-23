// https://leetcode.com/problems/min-cost-climbing-stairs/description
struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        //     Constraints:
        //
        // 2 <= cost.length <= 1000
        // 0 <= cost[i] <= 999
        let mut hi = cost[cost.len() - 1];
        let mut lo = cost[cost.len() - 2];
        for i in (0..cost.len() - 2).rev() {
            (lo, hi) = (cost[i] + i32::min(lo, hi), lo);
        }
        i32::min(lo, hi)
    }

    pub fn min_cost_climbing_stairs_classic(cost: Vec<i32>) -> i32 {
        //     Constraints:
        //
        // 2 <= cost.length <= 1000
        // 0 <= cost[i] <= 999

        let mut dp = vec![0i32; cost.len()];
        dp[cost.len() - 1] = cost[cost.len() - 1];
        dp[cost.len() - 2] = cost[cost.len() - 2];
        for i in (0..cost.len() - 2).rev() {
            dp[i] = cost[i] + i32::min(dp[i+1], dp[i+2]);
        }
        println!("{:?}", dp);
        i32::min(dp[0], dp[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_cost_climbing_stairs() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
        assert_eq!(55, Solution::min_cost_climbing_stairs(vec![55,372]));
        assert_eq!(850, Solution::min_cost_climbing_stairs(vec![313,850,812]));
        assert_eq!(850, Solution::min_cost_climbing_stairs_classic(vec![313,850,812]));
    }
}
