// #826
// https://leetcode.com/problems/most-profit-assigning-work/description/

struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn max_profit_assignment(
        mut difficulty: Vec<i32>,
        mut profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut work: Vec<(i32, i32)> = profit.drain(..).zip(difficulty.drain(..)).collect();
        work.sort_unstable_by_key(|(work_profit, _)| Reverse(*work_profit));

        worker.sort_unstable();
        let mut worker = &worker[..];

        let mut result = 0;

        for (profit, difficulty) in work.into_iter() {
            if worker.is_empty() {
                break;
            }

            match worker.binary_search(&(difficulty - 1)) {
                Ok(i) => {
                    result += (worker.len() - i - 1) as i32 * profit;
                    worker = &worker[..=i];
                }
                Err(i) => {
                    result += (worker.len() - i) as i32 * profit;
                    worker = &worker[..i];
                }
            }
        }

        result
    }

    pub fn max_profit_assignment_optimized(
        mut difficulty: Vec<i32>,
        mut profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut work: Vec<(i32, i32)> = difficulty.drain(..).zip(profit.drain(..)).collect();

        work.sort_unstable();
        worker.sort_unstable();

        let mut net_profit = 0;
        let mut max_profit = 0;
        let mut i = 0;

        for ability in worker {
            while i < work.len() && ability >= work[i].0 {
                max_profit = max_profit.max(work[i].1);
                i += 1;
            }

            net_profit += max_profit;
        }

        net_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_profit_assignig_work() {
        assert_eq!(
            448,
            Solution::max_profit_assignment_optimized(
                vec![5, 9, 16, 20, 34, 38, 38, 49, 75, 82],
                vec![18, 19, 20, 22, 29, 31, 48, 58, 67, 82],
                vec![37, 20, 8, 20, 17, 65, 86, 76, 44, 91]
            )
        );

        assert_eq!(
            100,
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            )
        );

        assert_eq!(
            90,
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 5, 7]
            )
        );

        assert_eq!(
            90,
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 5, 7]
            )
        );

        assert_eq!(
            0,
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25])
        );
    }
}
