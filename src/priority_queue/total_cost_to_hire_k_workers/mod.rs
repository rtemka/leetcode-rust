// https://leetcode.com/problems/total-cost-to-hire-k-workers/description
struct Solution;

use std::{cmp, collections::BinaryHeap};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        // priority queue of (value, index)
        let mut pq: BinaryHeap<cmp::Reverse<(i32, usize)>> = BinaryHeap::with_capacity(costs.len());
        let mut cost = 0;
        let (mut lo, mut hi) = (0, costs.len() - 1);
        // add candidates to queue
        let mut choosen = 0;
        while lo <= hi && choosen < candidates {
            if lo == hi {
                // case for the one candidate left
                pq.push(cmp::Reverse((costs[lo], lo)));
            } else {
                // average case
                pq.push(cmp::Reverse((costs[lo], lo)));
                pq.push(cmp::Reverse((costs[hi], hi)));
            }
            (lo, hi, choosen) = (lo + 1, hi.saturating_sub(1), choosen + 1);
        }
        for _ in 0..k {
            if let Some(cmp::Reverse((val, i))) = pq.pop() {
                cost += val as i64;
                if lo <= hi {
                    if i <= lo {
                        pq.push(cmp::Reverse((costs[lo], lo)));
                        lo += 1;
                    } else {
                        pq.push(cmp::Reverse((costs[hi], hi)));
                        hi -= 1;
                    }
                }
            }
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_cost_to_hire_k_workers() {
        assert_eq!(48, Solution::total_cost(vec![48], 1, 1));

        assert_eq!(
            423,
            Solution::total_cost(
                vec![31, 25, 72, 79, 74, 65, 84, 91, 18, 59, 27, 9, 81, 33, 17, 58],
                11,
                2
            )
        );
        assert_eq!(
            11,
            Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)
        );
        assert_eq!(4, Solution::total_cost(vec![1, 2, 4, 1], 3, 3));
    }
}
