// #1288
// https://leetcode.com/problems/remove-covered-intervals

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut covered = 0;
        let mut lo = 0;

        for hi in 1..intervals.len() {
            if intervals[lo][1] >= intervals[hi][1] {
                covered += 1;
            } else if intervals[lo][0] == intervals[hi][0] {
                covered += 1;
                lo = hi;
            } else {
                lo = hi;
            }
        }

        (intervals.len() - covered) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_covered_intervals() {
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]])
        );

        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]])
        );

        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]])
        );

        assert_eq!(1, Solution::remove_covered_intervals(vec![vec![1, 4]]));
    }
}
