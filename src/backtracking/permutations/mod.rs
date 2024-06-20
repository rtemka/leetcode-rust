use std::collections::HashSet;

// https://leetcode.com/problems/permutations/description/
struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Constraints:
        //  1 <= nums.length <= 6
        // -10 <= nums[i] <= 10
        // All the integers of nums are unique.
        let mut v = Vec::with_capacity((1..=nums.len()).product());
        Self::backtrack(
            &nums,
            &mut v,
            &mut Vec::with_capacity(nums.len()),
            &mut HashSet::with_capacity(nums.len()),
        );
        v
    }

    #[inline]
    fn backtrack(
        nums: &[i32],
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        set: &mut HashSet<i32>,
    ) {
        if nums.len() == cur.len() {
            res.push(cur.clone());
        } else {
            for &n in nums {
                if set.contains(&n) {
                    continue;
                }
                cur.push(n);
                set.insert(n);
                Self::backtrack(nums, res, cur, set);
                cur.pop();
                set.remove(&n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            Solution::permute(vec![1, 2, 3]),
        );

        assert_eq!(vec![vec![1]], Solution::permute(vec![1]));

        assert_eq!(vec![vec![0, 1], vec![1, 0]], Solution::permute(vec![0, 1]));

        assert_eq!(
            vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 4, 3],
                vec![1, 3, 2, 4],
                vec![1, 3, 4, 2],
                vec![1, 4, 2, 3],
                vec![1, 4, 3, 2],
                vec![2, 1, 3, 4],
                vec![2, 1, 4, 3],
                vec![2, 3, 1, 4],
                vec![2, 3, 4, 1],
                vec![2, 4, 1, 3],
                vec![2, 4, 3, 1],
                vec![3, 1, 2, 4],
                vec![3, 1, 4, 2],
                vec![3, 2, 1, 4],
                vec![3, 2, 4, 1],
                vec![3, 4, 1, 2],
                vec![3, 4, 2, 1],
                vec![4, 1, 2, 3],
                vec![4, 1, 3, 2],
                vec![4, 2, 1, 3],
                vec![4, 2, 3, 1],
                vec![4, 3, 1, 2],
                vec![4, 3, 2, 1],
            ],
            Solution::permute(vec![1, 2, 3, 4])
        );
    }
}
