// #90
// https://leetcode.com/problems/subsets-ii/description/

struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            start: usize,
            nums: &[i32],
            subset: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(subset.clone());

            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }

                subset.push(nums[i]);

                backtrack(i + 1, nums, subset, result);

                subset.pop();
            }
        }

        let mut result = Vec::new();
        let mut subset = Vec::new();

        nums.sort_unstable();

        backtrack(0, &nums, &mut subset, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_2() {
        assert_eq!(
            {
                let mut expected = vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![1, 2],
                    vec![1, 2, 2],
                    vec![2, 2],
                ];
                expected.sort_unstable();
                expected
            },
            {
                let mut res = Solution::subsets_with_dup(vec![1, 2, 2]);
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
                let mut res = Solution::subsets_with_dup(vec![0]);
                res.sort_unstable();
                res
            },
        );
    }
}
