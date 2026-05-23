use std::{collections::HashSet, ops::ControlFlow};

// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/description
struct Solution;

impl Solution {
    pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        banned.sort_unstable();
        let mut sum = 0;
        let mut count = 0;
        for i in 1..=n {
            if banned.binary_search(&i).is_ok() {
                continue;
            }
            sum += i;
            if sum > max_sum {
                break;
            } else {
                count += 1;
            }
        }
        count
    }

    pub fn max_count_iter(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let set: HashSet<i32> = banned.into_iter().collect();
        let mut sum = 0;
        match (1..=n)
            .filter(|x| !set.contains(x))
            .try_fold(0i32, |count, x| {
                sum += x;
                if sum > max_sum {
                    ControlFlow::Break(count)
                } else {
                    ControlFlow::Continue(count + 1)
                }
            }) {
            ControlFlow::Break(x) => x,
            ControlFlow::Continue(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_count_to_choose_from_range() {
        assert_eq!(7, Solution::max_count_iter(vec![11], 7, 50));
        assert_eq!(7, Solution::max_count(vec![11], 7, 50));
        assert_eq!(2, Solution::max_count(vec![1, 6, 5], 5, 6));
        assert_eq!(0, Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1));
        assert_eq!(124, Solution::max_count(vec![8108, 8155], 2431, 7821));
        assert_eq!(124, Solution::max_count_iter(vec![8108, 8155], 2431, 7821));
    }
}
