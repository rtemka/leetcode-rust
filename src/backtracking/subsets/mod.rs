// #78
// https://leetcode.com/problems/subsets/description/

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut subset = Vec::new();
        let mut visited = vec![false; nums.len()];

        fn backtrack(
            start: usize,
            nums: &[i32],
            visited: &mut [bool],
            subset: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(subset.clone());

            for i in start..nums.len() {
                if visited[i] {
                    continue;
                }

                subset.push(nums[i]);
                visited[i] = true;

                backtrack(i + 1, nums, visited, subset, result);

                visited[i] = false;
                subset.pop();
            }
        }

        backtrack(0, &nums, &mut visited, &mut subset, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets() {
        assert_eq!(
            {
                let mut expected = vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![1, 2],
                    vec![3],
                    vec![1, 3],
                    vec![2, 3],
                    vec![1, 2, 3],
                ];
                expected.sort_unstable();
                expected
            },
            {
                let mut res = Solution::subsets(vec![1, 2, 3]);
                res.sort_unstable();
                res
            },
        );

        assert_eq!(
            {
                let mut expected = vec![vec![0], vec![]];
                expected.sort_unstable();
                expected
            },
            {
                let mut res = Solution::subsets(vec![0]);
                res.sort_unstable();
                res
            },
        );
    }
}
