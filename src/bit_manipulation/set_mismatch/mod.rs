use std::collections::HashSet;

// https://leetcode.com/problems/set-mismatch/description/
struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        Self::find_error_nums_hash(nums)
    }

    pub fn find_error_nums_hash(nums: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(2);
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        let n = nums.len() as i32;
        let mut leftover = n * (n + 1) / 2; // sum of all numbers 1..n;
        for n in nums {
            if !set.insert(n) {
                v.push(n);
            } else {
                leftover -= n;
            }
        }
        v.push(leftover);
        v
    }

    pub fn find_error_nums_sort(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut v = Vec::from([0, 0]);
        let mut prev = 0;
        for i in 0..nums.len() {
            if nums[i] == prev {
                v[0] = nums[i];
            }
            if v[1] == 0 && nums[i] != i as i32 + 1 {
                v[1] = i as i32 + 1;
            }
            prev = nums[i];
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_error_nums() {
        assert_eq!(
            vec![3, 1],
            Solution::find_error_nums(vec![3, 2, 3, 4, 6, 5])
        );
        assert_eq!(vec![2, 1], Solution::find_error_nums(vec![2, 2]));
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }
}
