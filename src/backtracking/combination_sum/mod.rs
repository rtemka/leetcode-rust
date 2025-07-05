// #39
// https://leetcode.com/problems/combination-sum/description/
struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(&mut Vec::new(), 0, &candidates, target, &mut res);
        res
    }

    fn backtrack(
        comb: &mut Vec<i32>,
        start_index: usize,
        candidates: &Vec<i32>,
        target: i32,
        res: &mut Vec<Vec<i32>>,
    ) {
        // If the target is equal to 0, we found a combination that sums to initial target.
        if target == 0 {
            res.push(comb.clone());
            return;
        }
        // If the target is less than 0, no more valid
        // combinations can be created by adding to the current combination.
        if target < 0 {
            return;
        }
        // Starting from 'start_index', explore all combinations after adding 'candidates[i]'.
        for i in start_index..candidates.len() {
            // Add the current number to create a new combination.
            comb.push(candidates[i]);
            // Recursively explore all paths that branch from this new combination.
            Self::backtrack(comb, i, candidates, target - candidates[i], res);
            // Backtrack by removing the number we just added.
            comb.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination_sum() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            Solution::combination_sum(vec![2, 3, 6, 7], 7)
        );

        assert_eq!(
            vec![vec![1, 1, 1, 1], vec![1, 1, 2], vec![1, 3], vec![2, 2]],
            Solution::combination_sum(vec![1, 2, 3], 4)
        );
    }
}
