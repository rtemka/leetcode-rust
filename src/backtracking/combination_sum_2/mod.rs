// https://leetcode.com/problems/combination-sum-ii/description
struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut res = Vec::new();
        let mut cur = Vec::new();
        Self::backtrack(&candidates, &mut res, &mut cur, 0, target);
        res
    }

    fn backtrack(
        candidates: &[i32],
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        idx: usize,
        remaining: i32,
    ) {
        if remaining == 0 {
            res.push(cur.clone());
            return;
        }
        for i in idx..candidates.len() {
            if i > idx && candidates[i] == candidates[i - 1] {
                continue; // skip duplicates
            }
            let n = candidates[i];
            if n > remaining {
                break; // Stop if the current number is greater than the remaining target
            }
            cur.push(n);
            Self::backtrack(candidates, res, cur, i + 1, remaining - n);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination_sum2() {
        let mut expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        expected.sort_unstable();
        let mut ans = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        ans.sort_unstable();
        assert_eq!(expected, ans);
        // assert_eq!(
        //     vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        //     Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        // );

        let mut expected = vec![vec![1, 2, 2], vec![5]];
        expected.sort_unstable();
        let mut ans = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
        ans.sort_unstable();
        assert_eq!(expected, ans);
    }
}
