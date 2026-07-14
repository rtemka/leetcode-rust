// #2144
// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount

struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();

        cost.iter()
            .enumerate()
            .rev()
            .step_by(3)
            .fold(0, |acc, (i, _)| {
                acc + cost[i] + if i > 0 { cost[i - 1] } else { 0 }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_cost_of_bying_candies_with_discount() {
        assert_eq!(1, Solution::minimum_cost(vec![1]));
        assert_eq!(8, Solution::minimum_cost(vec![1, 2, 3, 4]));
        assert_eq!(5, Solution::minimum_cost(vec![1, 2, 3]));
        assert_eq!(23, Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]));
        assert_eq!(10, Solution::minimum_cost(vec![5, 5]));
    }
}
